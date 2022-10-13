namespace BubblesDivePlanner.Models.Cylinders
{
    public interface IGasManagement
    {
        ushort RemainingGas { get; }
        byte SurfaceAirConsumptionRate { get; }
        ushort UsedGas { get; }
        void UpdateGasUsage(IDiveStep diveStep);
    }
}