using BubblesDivePlanner.Models;

namespace BubblesDivePlanner.Controllers
{
    public class CylinderController : ICylinderController
    {
        public ushort CalculateInitialPressurisedVolume(byte cylinderVolume, ushort cylinderPressure) => (ushort)(cylinderPressure * cylinderVolume);
        public ushort CalculateGasUsage(byte surfaceAirConsumptionRate, IDiveStep diveStep) => (ushort)(((diveStep.Depth / 10) + 1) * diveStep.Time * surfaceAirConsumptionRate);
        public ushort CalculateRemainingGas(ushort remainingGas, ushort gasUsed) => (ushort)(remainingGas - gasUsed);
    }
}