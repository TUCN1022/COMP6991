use crate::connect::ConnectionWrite;
use crate::types::*;
use log::{error, info};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub struct IrcConnection {
    users_in_channel: HashMap<Channel, HashSet<Nick>>, // the sets of people that a channel have
    channels_for_user: HashMap<Nick, HashSet<Channel>>, //the sets of channel that a people have join in
    user_connection: HashMap<Nick, Arc<Mutex<ConnectionWrite>>>,
    all_user: HashSet<Nick>,       // the set of the all user in the connect
    all_channel: HashSet<Channel>, // the set of the all channel in the connect
}

impl Default for IrcConnection {
    fn default() -> Self {
        Self::new()
    }
}

impl IrcConnection {
    pub fn new() -> Self {
        Self {
            users_in_channel: HashMap::new(),
            channels_for_user: HashMap::new(),
            user_connection: HashMap::new(),
            all_user: HashSet::new(),
            all_channel: HashSet::new(),
        }
    }

    pub fn add_user(
        &mut self,
        nick: &Nick,
        writing: Arc<Mutex<ConnectionWrite>>,
    ) -> Result<(), ErrorType> {
        let insert_nick = nick.clone();
        if self.user_connection.contains_key(&insert_nick) {
            let _ = writing
                .lock()
                .unwrap()
                .write_message(format!(":{} 436 :Nickname collision\r\n", SERVER_NAME).as_str());
            return Err(ErrorType::NickCollision);
        }

        self.all_user.insert(insert_nick.clone());
        self.user_connection.insert(insert_nick, writing);

        Ok(())
    }

    pub fn remove_user(&mut self, nick: &Nick) {
        let remove_nick = &nick.clone();
        self.all_user.remove(&remove_nick.clone());
        for (nickname, channels) in &mut self.channels_for_user {
            if nickname == remove_nick {
                for channel in channels.iter() {
                    self.users_in_channel
                        .get_mut(channel)
                        .map(|nicks| nicks.remove(nick));
                }
                channels.clear();
                break;
            }
        }
        self.user_connection.remove(remove_nick);
    }

    pub fn add_user_to_channel(&mut self, nick: &Nick, channel: &Channel) -> Result<(), String> {
        let search_nick = nick.clone();
        self.all_channel.insert(channel.clone());
        if !self.user_connection.contains_key(&search_nick) {
            return Err(format!(
                "User who's nick is  {} not exist in the irc connection before adding to {}",
                search_nick, channel
            ));
        } else {
            self.users_in_channel
                .entry(channel.clone())
                .or_default()
                .insert(search_nick);
            self.channels_for_user
                .entry(nick.clone())
                .or_default()
                .insert(channel.clone());
        }
        Ok(())
    }

    pub fn remove_user_from_channel(
        &mut self,
        nick: &Nick,
        channel: &Channel,
    ) -> Result<(), String> {
        let search_nick = nick.clone();
        if !self.all_channel.contains(&channel.clone()) {
            let _err_message = format!(":{SERVER_NAME} 403 :No such channel").as_str();
            error!(
                "{}",
                format!(":{SERVER_NAME} 403 :No such channel").as_str()
            );
            return Err(format!(":{SERVER_NAME} 403 :No such channel"));
        } else if !self.user_connection.contains_key(&nick.clone()) {
            return Err(format!(
                "User who's nick is  {} not exist in the irc connection before remove from {}",
                nick.clone(),
                channel
            ));
        } else if let Some(users) = self.users_in_channel.get(channel) {
            if users.contains(&search_nick) {
                self.users_in_channel
                    .entry(channel.clone())
                    .or_default()
                    .remove(&nick.clone());
                self.channels_for_user
                    .entry(nick.clone())
                    .or_default()
                    .remove(&channel.clone());
                return Ok(());
            } else {
                error!(
                    "{}",
                    format!(":{SERVER_NAME} 401 :No such nick/channel").as_str()
                );
                return Err(format!(
                    "User who's nick is  {} not exist in the channel {}",
                    nick.clone(),
                    channel
                ));
            }
        }

        Ok(())
    }

    pub fn message_to_user(&mut self, nick: &Nick, message: &str) -> Result<(), ErrorType> {
        if !self.all_user.contains(&nick.clone()) {
            Err(ErrorType::NoSuchNick)
        } else if let Some(connection) = self.user_connection.get_mut(&nick.clone()) {
            let _ = connection
                .lock()
                .unwrap()
                .write_message(format!("{}\r\n", message.trim_end()).as_str());
            Ok(())
        } else {
            Err(ErrorType::NoSuchNick)
        }
    }

    pub fn message_to_channel(
        &mut self,
        channel: &Channel,
        message: &str,
    ) -> Result<(), ErrorType> {
        if !self.all_channel.contains(&channel.clone()) {
            Err(ErrorType::NoSuchChannel)
        } else {
            if let Some(nicks) = self.users_in_channel.get(&channel.clone()) {
                for nick in nicks.clone().iter() {
                    let _ = self.message_to_user(nick, message);
                }
            }
            Ok(())
        }
    }

    pub fn message_to_user_or_channel(
        &mut self,
        target: &Target,
        message: &str,
    ) -> Result<(), ErrorType> {
        match target {
            Target::User(nick) => self.message_to_user(nick, message),
            Target::Channel(channel) => self.message_to_channel(channel, message),
        }
    }
}

pub struct Origin {
    connection_wrtite: Arc<Mutex<ConnectionWrite>>,
}

pub struct HaveNick {
    nick: Nick,
}

pub struct HaveName {
    real_name: String,
    nick: Nick,
}

pub enum ClientState {
    Origin(Origin),
    HaveNick(HaveNick),
    HaveName(HaveName),
    Quit,
}

pub struct MessageHandler {
    state: ClientState,
    user_connections: Arc<Mutex<IrcConnection>>,
}

impl MessageHandler {
    pub fn new(
        user_connections: &Arc<Mutex<IrcConnection>>,
        connection_write: ConnectionWrite,
    ) -> Self {
        MessageHandler {
            state: ClientState::Origin(Origin {
                connection_wrtite: Arc::new(Mutex::new(connection_write)),
            }),
            user_connections: user_connections.clone(),
        }
    }

    pub fn has_quit(&self) -> bool {
        matches!(&self.state, ClientState::Quit)
    }

    pub fn get_nick(&self) -> Option<Nick> {
        match &self.state {
            ClientState::HaveNick(state) => Some(state.nick.clone()),
            ClientState::HaveName(state) => Some(state.nick.clone()),
            _ => None,
        }
    }

    pub fn handle_connection_lost(&mut self, nick: &Nick) {
        let mut user_conn_guard = self.user_connections.lock().unwrap();
        user_conn_guard.remove_user(nick);
        self.state = ClientState::Quit;
    }

    pub fn handle_parsed_message(&mut self, message: String) {
        let message = Some(message).as_deref().map(|msg| {
            let sender_nick = self.get_nick().unwrap_or_else(|| Nick("".to_owned()));
            ParsedMessage::try_from(UnparsedMessage {
                message: msg,
                sender_nick,
            })
        });

        match message {
            Some(Ok(message)) => match (&self.state, message.message) {
                (ClientState::Origin(state), Message::Nick(nick_message)) => {
                    let nick = nick_message.nick.clone();
                    let mut user_connections_add_nick = self.user_connections.lock().unwrap();
                    let add_user =
                        user_connections_add_nick.add_user(&nick, state.connection_wrtite.clone());
                    match add_user {
                        Err(err) => {
                            error!("{err}");
                            // self.state = ClientState::Quit;
                            // return;
                        }
                        Ok(_) => {
                            self.state = ClientState::HaveNick(HaveNick { nick });
                            info!(
                                "{}",
                                format!(": a client have set the NICK as {}", nick_message.nick)
                                    .as_str()
                            );
                        }
                    }
                }
                (ClientState::HaveNick(state), Message::User(user_message)) => {
                    let real_name = user_message.real_name.clone();
                    let mut user_connections_add_realname = self.user_connections.lock().unwrap();
                    let nick = &state.nick.clone();
                    let find_nick = &state.nick.clone();
                    let reply_message = &Reply::Welcome(WelcomeReply {
                        target_nick: nick.clone(),
                        message: format!("Hi {}, welcome to IRC", real_name),
                    })
                    .to_string();
                    let _ = user_connections_add_realname.message_to_user(find_nick, reply_message);
                    self.state = ClientState::HaveName(HaveName {
                        real_name: (real_name),
                        nick: (find_nick.clone()),
                    });

                    info!(
                        "{}",
                        format!(
                            ": a client have set the USER name  as {}",
                            user_message.real_name
                        )
                        .as_str()
                    );
                }
                (ClientState::HaveName(state), Message::Ping(ping_message)) => {
                    let mut user_connections_get_ping = self.user_connections.lock().unwrap();
                    let _nick = &state.nick.clone();
                    let message = ping_message;
                    let reply_message = &Reply::Pong(message).to_string();
                    let _ = user_connections_get_ping
                        .message_to_user(&state.nick.clone(), reply_message);
                    info!("{}", format!(": receive a PING message"));
                }
                (ClientState::HaveName(state), Message::Quit(quit_message)) => {
                    let mut user_connections_get_quit = self.user_connections.lock().unwrap();
                    let nick = &state.nick.clone();
                    let reply_message = &Reply::Quit(QuitReply {
                        message: quit_message,
                        sender_nick: nick.clone(),
                    })
                    .to_string();
                    let _ = user_connections_get_quit.message_to_user(nick, reply_message);

                    if let Some(quit_channel_set) = user_connections_get_quit
                        .channels_for_user
                        .get(&state.nick.clone())
                    {
                        for channel in quit_channel_set.clone() {
                            let _ = user_connections_get_quit
                                .message_to_channel(&channel, reply_message);
                            let _ =
                                user_connections_get_quit.remove_user_from_channel(nick, &channel);
                        }
                    }

                    self.state = ClientState::Quit;
                    user_connections_get_quit.remove_user(nick);
                    info!("{} quit out.", nick);
                }
                (ClientState::HaveName(state), Message::PrivMsg(priv_message)) => {
                    let mut user_connections_get_privmsg = self.user_connections.lock().unwrap();
                    let nick = &state.nick.clone();
                    let _name = &state.real_name.clone();
                    let target = &priv_message.target;

                    let reply_message = &Reply::PrivMsg(PrivReply {
                        message: priv_message.clone(),
                        sender_nick: nick.clone(),
                    })
                    .to_string();
                    let _getmsg = user_connections_get_privmsg
                        .message_to_user_or_channel(target, reply_message);
                    info!(
                        "{}",
                        format!(
                            ": {} send a PRIVMSg to {}",
                            state.nick.clone(),
                            priv_message.target.clone()
                        )
                        .as_str()
                    );
                    // match _getmsg {
                    //     Err(err) => {
                    //         error!("{err}");
                    //         let _error_message =
                    //             format!(":{SERVER_NAME} 401 :No such nick/channel").as_str();
                    //         let _give_back_errmsg = user_connections_get_privmsg.message_to_user(
                    //             &state.nick,
                    //             format!(":{SERVER_NAME} 401 :No such nick/channel").as_str(),
                    //         );
                    //         return;
                    //     }
                    //     Ok(_) => {}
                    // }
                    if let Err(err) = _getmsg {
                        error!("{err}");
                        let _error_message =
                            format!(":{SERVER_NAME} 401 :No such nick/channel").as_str();
                        let _give_back_errmsg = user_connections_get_privmsg.message_to_user(
                            &state.nick,
                            format!(":{SERVER_NAME} 401 :No such nick/channel").as_str(),
                        );
                        // return;
                    }
                }
                (ClientState::HaveName(state), Message::Join(join_message)) => {
                    let nick = &state.nick.clone();
                    let mut user_connections_get_joinmsg = self.user_connections.lock().unwrap();

                    let join_in_channel = &join_message.channel;
                    let _add_user =
                        user_connections_get_joinmsg.add_user_to_channel(nick, join_in_channel);
                    let reply_message = &Reply::Join(JoinReply {
                        message: (join_message.clone()),
                        sender_nick: (nick.clone()),
                    })
                    .to_string();
                    let _ = user_connections_get_joinmsg
                        .message_to_channel(join_in_channel, reply_message);
                    info!(
                        "{}",
                        format!(
                            ": {} join in a channel {}",
                            state.nick.clone(),
                            join_message.channel
                        )
                        .as_str()
                    );
                }
                (ClientState::HaveName(state), Message::Part(part_message)) => {
                    let mut user_connections_get_partmsg = self.user_connections.lock().unwrap();
                    let nick = &state.nick.clone();
                    let part_channel = &part_message.channel;
                    let _remove_user =
                        user_connections_get_partmsg.remove_user_from_channel(nick, part_channel);
                    match _remove_user {
                        Ok(_) => {
                            let reply_message = &Reply::Part(PartReply {
                                message: (part_message.clone()),
                                sender_nick: (nick.clone()),
                            })
                            .to_string();
                            let _ = user_connections_get_partmsg
                                .message_to_channel(part_channel, reply_message);
                            info!(
                                "{}",
                                format!(
                                    ": {} part out a channel {}",
                                    state.nick.clone(),
                                    part_message.channel.clone()
                                )
                                .as_str()
                            );
                        }
                        Err(not_find_message) => {
                            let _ = user_connections_get_partmsg
                                .message_to_user(&state.nick.clone(), not_find_message.as_str());
                            error!(
                                "{} is not in this channel, so it can not do PART command.",
                                state.nick.clone()
                            );
                        }
                    };
                }
                _ => {
                    error!("Not a correct command in this state.");
                }
            },

            Some(Err(err)) => {
                error!("{err}");
                if let Some(nick) = self.get_nick() {
                    let mut user_connection = self.user_connections.lock().unwrap();
                    let _ = user_connection.message_to_user(&nick, &err.to_string());
                } else if let ClientState::Origin(state) = &self.state {
                    let _ = state
                        .connection_wrtite
                        .lock()
                        .unwrap()
                        .write_message(format!("{}\r\n", err.to_string().trim_end()).as_str());
                }
            }

            None => {}
        };
    }
}
