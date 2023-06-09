extern crate alloc;

use tezos_smart_rollup_debug::debug_msg;
use tezos_smart_rollup_encoding::inbox::InboxMessage;
use tezos_smart_rollup_encoding::michelson::MichelsonUnit;
use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::runtime::Runtime;

mod counter;
use counter::*;

use tezos_smart_rollup_host::path::OwnedPath;
fn execute<Host: Runtime>(host: &mut Host, counter: Counter) -> Counter {
    // Read the input
    let input = host.read_input();

    match input {
        Err(_) | Ok(None) => counter,
        Ok(Some(message)) => {
            // if there is a messge let's proccess it
            host.write_debug("Hello message received");
            let data = message.as_ref();

            // let's decode the message

            match data {
                [0x00, ..] => {
                    host.write_debug("Message from the kernel.\n");
                    execute(host, counter)
                }
                [0x01, ..] => {
                    host.write_debug("Message from the user.\n");
                    // Let's skip the first byte of the data to get what the user has sent.
                    let user_message: Vec<&u8> = data.iter().skip(1).collect();
                    // We are parsing the message from the user.
                    // In the case of a good encoding we can process it.
                    let user_message = UserAction::try_from(user_message);
                    match user_message {
                        Ok(user_message) => {
                            let counter = transition(counter, user_message);
                            execute(host, counter)
                        }
                        Err(_) => execute(host, counter),
                    }
                }
                _ => execute(host, counter),
            }
        }
    }
}

fn entry(host: &mut impl Runtime) {
    let counter_path: OwnedPath = "/counter".as_bytes().to_vec().try_into().unwrap();

    let counter = Runtime::store_read(host, &counter_path, 0, 8)
        .map_err(|_| "Runtime error".to_string())
        .and_then(Counter::try_from)
        .unwrap_or_default();

    let counter = execute(host, counter);

    let counter: [u8; 8] = counter.into();

    let _ = Runtime::store_write(host, &counter_path, &counter, 0);
    host.mark_for_reboot().unwrap();
}

kernel_entry!(entry);

// To run:
// 1. cargo build --release --target wasm32-unknown-unknown --features greeter
// 2. octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/coutner_kernel.wasm --inputs ./counter_kernel/inputs.json
// 'load inputs'
// 'step result'
// 'show key /counter'

mod test {
    use super::*;

    #[test]
    fn test_counter() {
        let mut host = tezos_smart_rollup_mock::MockHost::default();

        let counter_path: OwnedPath = "/counter".as_bytes().to_vec().try_into().unwrap();
        host.run_level(entry);

        let counter = Runtime::store_read(&mut host, &counter_path, 0, 8)
            .map_err(|_| "Runtime error".to_string())
            .and_then(Counter::try_from)
            .unwrap_or_default();

        assert_eq!(counter, Counter { counter: 0 });

        let action = UserAction::Increment;
        host.add_external(action);
        host.run_level(entry);
        let counter = Runtime::store_read(&mut host, &counter_path, 0, 8)
            .map_err(|_| "Runtime error".to_string())
            .and_then(Counter::try_from)
            .unwrap_or_default();
        assert_eq!(counter, Counter { counter: 1 });
    }
}
