using BubblesDivePlanner.Models;
using BubblesDivePlanner.Models.Cylinders;

namespace BubblesDivePlanner.Controllers
{
    public interface ICylinderController
    {
        ushort CalculateInitialPressurisedVolume(byte cylinderVolume, ushort cylinderPressure);
        ushort CalculateGasUsage(byte surfaceAirConsumptionRate, IDiveStep diveStep);
        ushort CalculateRemainingGas(ushort remainingGas, ushort gasUsed);
    }
}