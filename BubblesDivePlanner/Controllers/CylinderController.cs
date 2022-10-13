using BubblesDivePlanner.Models;

namespace BubblesDivePlanner.Controllers
{
    public class CylinderController : ICylinderController
    {
        public ushort CalculateGasUsage(byte surfaceAirConsumptionRate, IDiveStep diveStep) => (ushort)(((diveStep.Depth / 10) + 1) * diveStep.Time * surfaceAirConsumptionRate);
        public ushort CalculateRemainingGas(ushort remainingGas, ushort gasUsed) => gasUsed < remainingGas ? (ushort)(remainingGas - gasUsed) : (ushort)0;
    }
}