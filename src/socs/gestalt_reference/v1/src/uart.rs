pub trait GestaltUart
{
    type TxPin;
    type RxPin;
    type Baud;
    type TxBuf;
    type RxBuf;

    fn set_rx       (&self, _: Self::RxPin);
    fn set_tx       (&self, _: Self::TxPin);
    fn set_baud     (&self, _: Self::Baud);
    fn set_tx_buf   (&self, _: &[Self::TxBuf]);
    fn set_rx_buf   (&self, _: &[Self::RxBuf]);

}