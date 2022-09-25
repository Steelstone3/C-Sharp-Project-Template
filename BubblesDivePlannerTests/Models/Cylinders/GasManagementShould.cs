using BubblesDivePlanner.Models;
using BubblesDivePlanner.Models.Cylinders;
using Xunit;

namespace BubblesDivePlannerTests.Models.Cylinders
{
    public class GasManagementShould
    {
        private readonly ushort remainingGas = 2400;
        private readonly byte surfaceAirConsumptionRate = 12;

        [Fact]
        public void ConstructAGasManagement()
        {
            IGasManagement gasManagement = new GasManagement(remainingGas, surfaceAirConsumptionRate);

            Assert.Equal(remainingGas, gasManagement.RemainingGas);
            Assert.Equal(surfaceAirConsumptionRate, gasManagement.SurfaceAirConsumptionRate);
            Assert.Equal(0, gasManagement.GasUsed);
        }

        [Fact]
        public void UpdateGasUsage()
        {
            IDiveStep diveStep = new DiveStep(50, 10);
            IGasManagement gasManagement = new GasManagement(remainingGas, surfaceAirConsumptionRate);

            gasManagement.UpdateGasUsage(diveStep);

            Assert.Equal(surfaceAirConsumptionRate, gasManagement.SurfaceAirConsumptionRate);
            Assert.Equal(1680, gasManagement.RemainingGas);
            Assert.Equal(720, gasManagement.GasUsed);
        }
    }
}