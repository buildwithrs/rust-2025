use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(100);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("received x: {}", msg);
        }
    });

    let mut recv_task = tokio::spawn(async move {
        for x in 0..10 {
            let _ = tx.send(x);
            println!("send x: {}", x);
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };
}
