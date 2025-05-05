pub mod relayer {
    use solana_sdk::pubkey::Pubkey;

    pub struct Relayer {
        pub keypair: Keypair,
        pub rpc_client: RpcClient,
    }

    impl Relayer {
        pub fn new(keypair: Keypair, rpc_url: &str) -> Self {
            let rpc_client = RpcClient::new(rpc_url);
            Self { keypair, rpc_client }
        }

        pub async fn send_transaction(&self, transaction: Transaction) -> Result<Signature, Box<dyn std::error::Error>> {
            let signature = self.rpc_client.send_transaction(&transaction).await?;
            Ok(signature)
        }

        // Additional methods for handling gasless transactions
    }
      }
