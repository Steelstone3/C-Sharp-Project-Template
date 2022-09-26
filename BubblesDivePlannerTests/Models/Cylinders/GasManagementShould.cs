using BubblesDivePlanner.Controllers;
using BubblesDivePlanner.Models;
using BubblesDivePlanner.Models.Cylinders;
using Moq;
using Xunit;

namespace BubblesDivePlannerTests.Models.Cylinders
{
    public class GasManagementShould
    {
        private IGasManagement gasManagement;
        private readonly ICylinderController cylinderController = new CylinderController();
        private readonly ushort remainingGas = 2400;
        private readonly byte surfaceAirConsumptionRate = 12;
        private readonly IDiveStep diveStep = new DiveStep(50, 10);

        public GasManagementShould()
        {
            gasManagement = new GasManagement(cylinderController, remainingGas, surfaceAirConsumptionRate);
        }

        [Fact]
        public void ContainsRemainingGas()
        {
            Assert.Equal(remainingGas, gasManagement.RemainingGas);
        }

        [Fact]
        public void ContainsGasUsed()
        {
            Assert.Equal(0, gasManagement.GasUsed);
        }

        [Fact]
        public void ContainsSurfaceAirConsumptionRate()
        {
            Assert.Equal(surfaceAirConsumptionRate, gasManagement.SurfaceAirConsumptionRate);
        }

        [Fact]
        public void UpdateGasUsage()
        {
            var mockCylinderController = MockCylinderController();
            gasManagement = new GasManagement(mockCylinderController.Object, remainingGas, surfaceAirConsumptionRate);

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