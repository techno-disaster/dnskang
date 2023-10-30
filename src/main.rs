mod primitives;

use std::net::UdpSocket;

/// Handle a single incoming packet
fn handle_query(socket: &UdpSocket) -> primitives::Result<()> {
    let mut req_buffer = primitives::BytePacketBuffer::new();

    // store src addr
    let (_, src) = socket.recv_from(&mut req_buffer.buf)?;

    let mut request = primitives::DnsPacket::from_buffer(&mut req_buffer)?;

    let mut packet = primitives::DnsPacket::new();
    packet.header.id = request.header.id;

    // In the normal case, exactly one question is present
    if let Some(question) = request.questions.pop() {
        println!("Received query: {:?}", question);
        if question.name == "dnskang" {
            let question: String = "CUSTOM_DATA_QUESTION".into();
            let answer: String = "CUSTOM_DATA_ANSWER".into();
            packet.answers.push(primitives::DnsRecord::CNAME {
                domain: question,
                host: answer,
                ttl: 1,
            })
        }
        // source: https://github.com/EmilHernvall/dnsguide/blob/master/examples/sample4.rs#L730
        else if let Ok(result) = primitives::lookup(&question.name, question.qtype) {
            packet.questions.push(question);
            packet.header.rescode = result.header.rescode;

            for rec in result.answers {
                println!("Answer: {:?}", rec);
                packet.answers.push(rec);
            }
            for rec in result.authorities {
                println!("Authority: {:?}", rec);
                packet.authorities.push(rec);
            }
            for rec in result.resources {
                println!("Resource: {:?}", rec);
                packet.resources.push(rec);
            }
        } else {
            packet.header.rescode = primitives::ResultCode::SERVFAIL;
        }
    } else {
        packet.header.rescode = primitives::ResultCode::FORMERR;
    }

    let mut res_buffer = primitives::BytePacketBuffer::new();
    packet.write(&mut res_buffer)?;

    let len = res_buffer.pos();
    let data = res_buffer.get_range(0, len)?;

    socket.send_to(data, src)?;

    Ok(())
}

fn main() -> primitives::Result<()> {
    let socket = UdpSocket::bind(("127.0.0.1", 20053))?; // ideally 0.0.0.0:53

    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
