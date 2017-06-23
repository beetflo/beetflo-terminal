
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::path::PathBuf;
use std::{fmt, mem};
use std;

use clap::{ArgMatches, App};

#[derive(Debug, Clone)]
pub struct Environment<'a> {
    pub input: u32,
    pub output: u32,
    pub matches: ArgMatches<'a>
}

#[derive(Clone)]
pub struct EnvironmentSingleton<'a> {
    pub inner: Arc<Mutex<Environment<'a>>>,
}

// Initialize it to a null value
static mut SINGLETON: *const EnvironmentSingleton = 0 as *const EnvironmentSingleton;
static ONCE: Once = ONCE_INIT;


pub fn singleton() -> EnvironmentSingleton<'static> {
    unsafe {
        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

impl<'a> Environment<'a> {
    pub fn global() -> Environment<'static> {
        let inner: &Arc<Mutex<Environment<'static>>> = &singleton().inner;
        let copy = inner.lock().unwrap().clone();
        copy
    }

    pub fn init() {
        let matches = App::new("Beetflo")
            .version("v1.0-beta")
            .get_matches();


        // Handle dynamic barge root based on new project creation
        let input = match matches.occurrences_of("midi-input") {
            1 => {
                matches.value_of("midi-input").unwrap().parse::<u32>().unwrap()
            }

            _ => 1
        };

        let output = match matches.occurrences_of("midi-output") {
            1 => {
                matches.value_of("midi-output").unwrap().parse::<u32>().unwrap()
            }

            _ => 1
        };

        let env = Environment {
            input: input,
            output: output,
            matches: matches.clone(),
        };

        unsafe {
            ONCE.call_once(|| {
                // Make it
                let singleton = EnvironmentSingleton { inner: Arc::new(Mutex::new(env)) };

                // Put it in the heap so it can outlive this call
                SINGLETON = mem::transmute(Box::new(singleton));
            });
        }
    }
}
