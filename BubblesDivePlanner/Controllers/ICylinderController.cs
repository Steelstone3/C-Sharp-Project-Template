using BubblesDivePlanner.Models;

namespace BubblesDivePlanner.Controllers
{
    public interface ICylinderController
    {
        ushort CalculateGasUsage(byte surfaceAirConsumptionRate, IDiveStep diveStep);
        ushort CalculateRemainingGas(ushort remainingGas, ushort gasUsed);
    }
}