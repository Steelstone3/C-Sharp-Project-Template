namespace BubblesDivePlanner.Models.Cylinders
{
    public interface IGasManagement
    {
        ushort RemainingGas { get; }
        byte SurfaceAirConsumptionRate { get; }
        ushort GasUsed { get; }
        void UpdateGasUsage(IDiveStep diveStep);
    }
}