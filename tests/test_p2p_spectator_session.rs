use ggrs::{PlayerType, SessionState};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use serial_test::serial;

mod stubs;

#[test]
#[serial]
fn test_create_session() {
    let host_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 7777);
    assert!(ggrs::start_p2p_spectator_session(1, stubs::INPUT_SIZE, 9999, host_addr).is_ok());
}

#[test]
#[serial]
fn test_start_session() {
    let host_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 7777);
    let mut spec_sess =
        ggrs::start_p2p_spectator_session(1, stubs::INPUT_SIZE, 9999, host_addr).unwrap();
    assert!(spec_sess.start_session().is_ok());
    assert!(spec_sess.current_state() == SessionState::Synchronizing);
}

#[test]
#[serial]
fn test_synchronize_with_host() {
    let host_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 7777);
    let spec_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8888);

    let mut host_sess = ggrs::start_p2p_session(1, stubs::INPUT_SIZE, 7777).unwrap();
    let mut spec_sess =
        ggrs::start_p2p_spectator_session(1, stubs::INPUT_SIZE, 8888, host_addr).unwrap();

    host_sess.add_player(PlayerType::Local, 0).unwrap();
    host_sess
        .add_player(PlayerType::Spectator(spec_addr), 2)
        .unwrap();

    host_sess.start_session().unwrap();
    spec_sess.start_session().unwrap();

    assert_eq!(spec_sess.current_state(), SessionState::Synchronizing);
    assert_eq!(host_sess.current_state(), SessionState::Synchronizing);

    for _ in 0..10 {
        spec_sess.poll_remote_clients();
        host_sess.poll_remote_clients();
    }

    assert_eq!(spec_sess.current_state(), SessionState::Running);
    assert_eq!(host_sess.current_state(), SessionState::Running);
}
