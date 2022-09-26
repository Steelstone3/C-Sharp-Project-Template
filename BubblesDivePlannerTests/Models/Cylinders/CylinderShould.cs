using BubblesDivePlanner.Controllers;
using BubblesDivePlanner.Models.Cylinders;
using Moq;
using Xunit;

namespace BubblesDivePlannerTests.Models.Cylinders
{
    public class CylinderShould
    {
        private readonly byte cylinderVolume = 12;
        private readonly ushort cylinderPressure = 200;
        private readonly ushort expectedInitialPressurisedVolume = 2400;
        private readonly byte surfaceAirConsumptionRate = 12;
        private readonly ICylinderController cylinderController = new CylinderController();
        private readonly Mock<IGasMixture> dummyGasMixture = new();
        private ICylinder cylinder;

        public CylinderShould()
        {
            cylinder = new Cylinder(cylinderController, cylinderVolume, cylinderPressure, dummyGasMixture.Object, surfaceAirConsumptionRate);
        }

        [Fact]
        public void ContainsCylinderVolume()
        {
            Assert.Equal(cylinderVolume, cylinder.CylinderVolume);
        }

        [Fact]
        public void ContainsCylinderPressure()
        {
            Assert.Equal(cylinderPressure, cylinder.CylinderPressure);
        }

        [Fact]
        public void ContainsInitialPressurisedVolume()
        {
            Assert.Equal(expectedInitialPressurisedVolume, cylinder.InitialPressurisedVolume);
        }

        [Fact]
        public void ContainsGasMixture()
        {
            Assert.NotNull(cylinder.GasMixture);
        }

        [Fact]
        public void ContainsGasManagement()
        {
            Assert.NotNull(cylinder.GasManagement);
        }

        [Fact]
        public void CalculateInitialPressurisedVolume()
        {
            var mockCylinderController = MockCylinderController();

            cylinder = new Cylinder(mockCylinderController.Object, cylinderVolume, cylinderPressure, dummyGasMixture.Object, surfaceAirConsumptionRate);

            mockCylinderController.VerifyAll();
        }

        private Mock<ICylinderController> MockCylinderController()
        {
            var mockCylinderController = new Mock<ICylinderController>();
            mockCylinderController.Setup(cc => cc.CalculateInitialPressurisedVolume(cylinderVolume, cylinderPressure)).Returns(expectedInitialPressurisedVolume);
            return mockCylinderController;
        }
    }
}