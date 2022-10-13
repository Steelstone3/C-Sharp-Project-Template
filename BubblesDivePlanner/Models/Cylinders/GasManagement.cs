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
        public ushort UsedGas { get; private set; }

        public void UpdateGasUsage(IDiveStep diveStep)
        {
            UsedGas = (ushort)(((diveStep.Depth / 10) + 1) * diveStep.Time * SurfaceAirConsumptionRate);
            RemainingGas = UsedGas < RemainingGas ? (ushort)(RemainingGas - UsedGas) : (ushort)0;
        }
    }
}