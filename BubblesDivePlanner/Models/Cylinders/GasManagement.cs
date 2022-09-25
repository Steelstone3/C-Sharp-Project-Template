namespace BubblesDivePlanner.Models.Cylinders
{
    public class GasManagement : IGasManagement
    {
        public GasManagement(ushort initialPressurisedVolume, byte surfaceAirConsumptionRate)
        {
            RemainingGas = initialPressurisedVolume;
            SurfaceAirConsumptionRate = surfaceAirConsumptionRate;
        }

        public ushort RemainingGas { get; private set; }
        public byte SurfaceAirConsumptionRate { get; }
        public ushort GasUsed { get; private set;}

        //TODO Pass this logic down to a controller layer
        public void UpdateGasUsage(IDiveStep diveStep)
        {
            GasUsed = (ushort)(((diveStep.Depth / 10) + 1) * diveStep.Time * SurfaceAirConsumptionRate);
            RemainingGas -= GasUsed;
        }
    }
}