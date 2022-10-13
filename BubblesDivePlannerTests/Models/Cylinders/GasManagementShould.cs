using BubblesDivePlanner.Models;
using BubblesDivePlanner.Models.Cylinders;
using Xunit;

namespace BubblesDivePlannerTests.Models.Cylinders
{
    public class GasManagementShould
    {
        private IGasManagement gasManagement;
        private readonly ushort remainingGas = 2400;
        private readonly byte surfaceAirConsumptionRate = 12;

        public GasManagementShould()
        {
            gasManagement = new GasManagement(remainingGas, surfaceAirConsumptionRate);
        }

        [Fact]
        public void ContainsRemainingGas()
        {
            Assert.Equal(remainingGas, gasManagement.RemainingGas);
        }

        [Fact]
        public void ContainsGasUsed()
        {
            Assert.Equal(0, gasManagement.UsedGas);
        }

        [Fact]
        public void ContainsSurfaceAirConsumptionRate()
        {
            Assert.Equal(surfaceAirConsumptionRate, gasManagement.SurfaceAirConsumptionRate);
        }

        [Theory]
        [InlineData(2400, 12, 50, 10, 720, 1680)]
        [InlineData(3600, 24, 50, 10, 1440, 2160)]
        [InlineData(0, 12, 100, 10, 1320, 0)]
        [InlineData(4800, 12, 50, 20, 1440, 3360)]
        [InlineData(1000, 0, 50, 10, 0, 1000)]
        [InlineData(3000, 12, 0, 10, 120, 2880)]
        [InlineData(4800, 12, 50, 0, 0, 4800)]
        [InlineData(2400, 12, 0, 0, 0, 2400)]
        [InlineData(0, 0, 0, 0, 0, 0)]
        public void CalculateGasUsage(ushort remainingGas, byte surfaceAirConsumptionRate, byte depth, byte time, ushort expectedUsedGas, ushort expectedRemainingGas)
        {
            gasManagement = new GasManagement(remainingGas, surfaceAirConsumptionRate);
            IDiveStep diveStep = new DiveStep(depth, time);

            gasManagement.UpdateGasUsage(diveStep);

            Assert.Equal(expectedUsedGas, gasManagement.UsedGas);
            Assert.Equal(expectedRemainingGas, gasManagement.RemainingGas);
        }
    }
}