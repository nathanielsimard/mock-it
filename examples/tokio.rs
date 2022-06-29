use std::time::Duration;
use tokio::time::sleep;

// #[cfg_attr(test, mock_it::mock_it)]
#[mock_it::mock_it]
#[async_trait::async_trait]
trait Heart {
    async fn beat(&self) -> Result<bool, String>;
}

struct Jogger<H: Heart> {
    pub heart: H,
}

impl<H: Heart> Jogger<H> {
    fn new(heart: H) -> Self {
        Self { heart }
    }

    async fn run(self) {
        loop {
            let status = self.heart.beat().await.unwrap();
            match status {
                true => sleep(Duration::from_secs(1)).await,
                false => break,
            };
        }
    }
}

// #[cfg(test)]
pub mod tests {
    use super::*;

    // #[tokio::test]
    pub async fn should_run_as_long_as_it_is_not_finished() {
        let heart = HeartMock::new();
        heart.when_beat().will_return(Ok(true));

        let jogger = Jogger::new(heart.clone());
        let handle = tokio::spawn(async move {
            jogger.run().await;
        });
        assert!(!handle.is_finished());

        heart.when_beat().will_return(Ok(false));
        sleep(Duration::from_millis(100)).await;
        assert!(handle.is_finished());
    }
}

fn main() {
    let _ = tests::should_run_as_long_as_it_is_not_finished();
}
