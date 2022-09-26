using BubblesDivePlanner.Controllers;
using BubblesDivePlanner.Models;
using Xunit;

namespace BubblesDivePlannerTests.Controllers
{
    public class CylinderControllerShould
    {
        private readonly ICylinderController cylinderController = new CylinderController();

        [Theory]
        [InlineData(12, 200, 2400)]
        public void CalculateInitialPressurisedVolume(byte cylinderVolume, ushort cylinderPressure, ushort expectedInitialPressurisedVolume)
        {
            var initialPressurisedVolume = cylinderController.CalculateInitialPressurisedVolume(cylinderVolume, cylinderPressure);

            Assert.Equal(expectedInitialPressurisedVolume, initialPressurisedVolume);
        }

        [Theory]
        [InlineData(12, 50, 10, 720)]
        public void CalculateGasUsage(byte surfaceAirConsumptionRate, byte depth, byte time, ushort expectedGasUsage)
        {
            IDiveStep diveStep = new DiveStep(depth, time);

            var gasUsed = cylinderController.CalculateGasUsage(surfaceAirConsumptionRate, diveStep);

            Assert.Equal(expectedGasUsage, gasUsed);
        }

        [Theory]
        [InlineData(2400, 720, 1680)]
        public void CalculateRemainingGas(ushort previousGasRemaining, ushort gasUsed, ushort expectedRemainingGas)
        {
            var gasRemaining = cylinderController.CalculateRemainingGas(previousGasRemaining, gasUsed);

            Assert.Equal(expectedRemainingGas, gasRemaining);
        }
    }
}