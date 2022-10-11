/* There are four kinds of tokio channels:
   - multi-producer,  single-consumer (tokio::sync::mpsc)
   - multi-producer,  multi-consumer  (tokio::sync::broadcast)
   - single-producer, single-consumer (tokio::sync::oneshot)
   - single-producer, multi-consumer  (tokio::sync::watch)
*/

#[cfg(test)]
mod tests {
	#[tokio::test]
	async fn oneshot() {
		let value_to_send = 1;
		let (tx, rx) = tokio::sync::oneshot::channel::<i32>();  // ::<> Turbofish! ::<>

		let receiver = tokio::spawn(async move {
			println!("rx.await");
			match rx.await {
				Ok(value) => {
					println!("rx received {}", value);
					assert_eq!(value_to_send, value);
				}
				Err(_) => panic!("Sender was dropped!"),
			}
		});

		tokio::spawn(async move {
			println!("tx.send({})", value_to_send);
			if let Err(_) = tx.send(value_to_send) {
				panic!("Receiver was dropped!")
			}
		});

		println!("awaiting receiver");
		receiver.await.unwrap();
	}
}
