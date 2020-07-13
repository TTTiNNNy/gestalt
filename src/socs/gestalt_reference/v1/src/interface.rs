
pub trait GestaltInerface
{
    fn write(&self);
    fn read(&self);
}

pub trait DMAGestaltInterface
{
    fn dma_write(&self);
    fn dma_read(&self);
}

pub trait NonDMAGestaltInterface
{
    fn non_dma_write(&self);
    fn non_dma_read(&self);
}