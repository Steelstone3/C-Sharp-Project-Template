using BubblesDivePlanner.Controllers;
using BubblesDivePlanner.Models;
using BubblesDivePlanner.Models.Cylinders;
using Moq;
using Xunit;

namespace BubblesDivePlannerTests.Models.Cylinders
{
    public class GasManagementShould
    {
        private readonly IGasManagement gasManagement;
        private readonly Mock<ICylinderController> mockCylinderController;
        private readonly ushort remainingGas = 2400;
        private readonly byte surfaceAirConsumptionRate = 12;
        private readonly IDiveStep diveStep = new DiveStep(50, 10);

        public GasManagementShould()
        {
            mockCylinderController = MockCylinderController();
            gasManagement = new GasManagement(mockCylinderController.Object, remainingGas, surfaceAirConsumptionRate);
        }

        [Fact]
        public void ConstructAGasManagement()
        {
            IGasManagement gasManagement = new GasManagement(mockCylinderController.Object, remainingGas, surfaceAirConsumptionRate);

            Assert.Equal(remainingGas, gasManagement.RemainingGas);
            Assert.Equal(surfaceAirConsumptionRate, gasManagement.SurfaceAirConsumptionRate);
            Assert.Equal(0, gasManagement.GasUsed);
        }

        [Fact]
        public void UpdateGasUsage()
        {
            gasManagement.UpdateGasUsage(diveStep);

            mockCylinderController.VerifyAll();
        }

        private Mock<ICylinderController> MockCylinderController()
        {
            var mockCylinderController = new Mock<ICylinderController>();
            mockCylinderController.Setup(cc => cc.CalculateGasUsage(surfaceAirConsumptionRate, diveStep));
            mockCylinderController.Setup(cc => cc.CalculateRemainingGas(remainingGas, 0));
            return mockCylinderController;
        }
    }
}