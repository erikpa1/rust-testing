use std::sync::{Arc, Mutex};

use std::collections::HashMap;

use std::collections::HashSet;

// static GLOBAL_DATA: Mutex<Option<HashSet<i32>>> = Mutex::new(None);

// fn main() {
//     *GLOBAL_DATA.lock().unwrap() = Some(HashSet::from([42]));
//     println!("V2: {:?}", GLOBAL_DATA.lock().unwrap());
// }

struct StructHelper {
    pub value: Option<Arc<SessionsManager>>,
}

static SESSIONS: Mutex<StructHelper> = Mutex::new(StructHelper {
    value: Option::None,
});

pub struct Session {
    id: String,
}

pub struct SessionsManager {
    sessions: HashMap<String, Session>,
}

impl SessionsManager {
    pub fn new() -> SessionsManager {
        return SessionsManager {
            sessions: HashMap::new(),
        };
    }

    pub fn add_sesion(&mut self, session_id: String) {
        self.sessions.insert(
            session_id.clone(),
            Session {
                id: session_id.clone(),
            },
        );
    }

    pub fn get_sessions_count(&self) -> usize {
        return self.sessions.len();
    }
}

pub struct SessionManagerInstance {}

impl SessionManagerInstance {
    pub fn init_singleton() {
        SESSIONS.lock().unwrap().value = Option::Some(Arc::new(SessionsManager::new()));
    }

    pub fn get_sesions_count() -> usize {
        let mut mutex_guard = SESSIONS.lock().unwrap();

        match &mut mutex_guard.value {
            Some(session_manager) => return session_manager.get_sessions_count(),
            None => {}
        }

        return 0;
    }

    // pub fn add_session(session_id: String) {
    //     let mut session_manager = SESSIONS.lock().unwrap();

    //     session_manager.value.unwrap().add_sesion(session_id);
    // }
}
