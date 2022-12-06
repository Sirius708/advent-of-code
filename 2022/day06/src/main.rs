use std::collections::HashSet;

fn main() {
    let input = util::get_input_lines().remove(0);

    const PACKET_MARKER_LEN: usize = 4;
    const MESSAGE_MARKER_LEN: usize = 14;

    for i in 0..(input.len() - PACKET_MARKER_LEN) {
        let possible_marker = &input[i..(i + PACKET_MARKER_LEN)];
        if possible_marker.chars().collect::<HashSet<_>>().len() == PACKET_MARKER_LEN {
            println!(
                "First packet maker after {} characters",
                i + PACKET_MARKER_LEN
            );
            break;
        }
    }

    for i in 0..(input.len() - MESSAGE_MARKER_LEN) {
        let possible_marker = &input[i..(i + MESSAGE_MARKER_LEN)];
        if possible_marker.chars().collect::<HashSet<_>>().len() == MESSAGE_MARKER_LEN {
            println!(
                "First message maker after {} characters",
                i + MESSAGE_MARKER_LEN
            );
            break;
        }
    }
}
