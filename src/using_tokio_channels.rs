/* There are four kinds of tokio channels:
   - multi-producer,  single-consumer (tokio::sync::mpsc)
   - multi-producer,  multi-consumer  (tokio::sync::broadcast)
   - single-producer, single-consumer (tokio::sync::oneshot)
   - single-producer, multi-consumer  (tokio::sync::watch)
*/
use tokio::sync::oneshot::channel;

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn oneshot() {
		let (tx, rx) = channel::<i32>();  // ::<> Turbofish! ::<>

		let receiver = tokio::spawn(async move {
			match rx.await {
				Ok(value) => assert_eq!(1, value),
				Err(_) => panic!("Sender was dropped!"),
			}
		});

		tokio::spawn(async move {
			if let Err(_) = tx.send(1) {
				panic!("Receiver was dropped!")
			}
		});

		receiver.await.unwrap();

	}
}
