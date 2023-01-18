use std::sync::Mutex;

use std::collections::HashMap;

struct StructHelper {
    pub value: Option<SessionsManager>,
}

static ARRAY: Mutex<StructHelper> = Mutex::new(StructHelper {
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

    pub fn init_singleton() {
        ARRAY.lock().unwrap().value = Option::Some(SessionsManager::new());
    }

    pub fn get_sesions_count() -> usize {
        let mut mutex_guard = ARRAY.lock().unwrap();

        match &mut mutex_guard.value {
            Some(session_manager) => return session_manager.get_sessions_count(),
            None => {}
        }

        return 0;
    }

    // pub const fn new_const() -> SessionsManager {
    //     return SessionsManager {
    //         sessions: HashMap::new(),
    //     };
    // }

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
