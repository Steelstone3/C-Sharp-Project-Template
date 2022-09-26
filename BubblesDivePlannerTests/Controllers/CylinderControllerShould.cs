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
        [InlineData(24, 200, 4800)]
        [InlineData(12, 400, 4800)]
        [InlineData(0, 200, 0)]
        [InlineData(12, 0, 0)]
        [InlineData(0, 0, 0)]
        public void CalculateInitialPressurisedVolume(byte cylinderVolume, ushort cylinderPressure, ushort expectedInitialPressurisedVolume)
        {
            var initialPressurisedVolume = cylinderController.CalculateInitialPressurisedVolume(cylinderVolume, cylinderPressure);

            Assert.Equal(expectedInitialPressurisedVolume, initialPressurisedVolume);
        }

        [Theory]
        [InlineData(12, 50, 10, 720)]
        [InlineData(24, 50, 10, 1440)]
        [InlineData(12, 100, 10, 1320)]
        [InlineData(12, 50, 20, 1440)]
        [InlineData(0, 50, 10, 0)]
        [InlineData(12, 0, 10, 120)]
        [InlineData(12, 50, 0, 0)]
        [InlineData(12, 0, 0, 0)]
        [InlineData(0, 0, 0, 0)]
        public void CalculateGasUsage(byte surfaceAirConsumptionRate, byte depth, byte time, ushort expectedGasUsage)
        {
            IDiveStep diveStep = new DiveStep(depth, time);

            var gasUsed = cylinderController.CalculateGasUsage(surfaceAirConsumptionRate, diveStep);

            Assert.Equal(expectedGasUsage, gasUsed);
        }

        [Theory]
        [InlineData(2400, 720, 1680)]
        [InlineData(1000, 500, 500)]
        [InlineData(500, 720, 0)]
        [InlineData(0, 720, 0)]
        public void CalculateRemainingGas(ushort previousGasRemaining, ushort gasUsed, ushort expectedRemainingGas)
        {
            var gasRemaining = cylinderController.CalculateRemainingGas(previousGasRemaining, gasUsed);

            Assert.Equal(expectedRemainingGas, gasRemaining);
        }
    }
}