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

        [Fact]
        public void ConstructACylinder()
        {
            var stubCylinderController = StubCylinderController();

            Mock<IGasMixture> dummyGasMixture = new();
            byte surfaceAirConsumptionRate = 12;
            ICylinder cylinder = new Cylinder(stubCylinderController.Object, cylinderVolume, cylinderPressure, dummyGasMixture.Object, surfaceAirConsumptionRate);

            stubCylinderController.VerifyAll();
            Assert.Equal(cylinderVolume, cylinder.CylinderVolume);
            Assert.Equal(cylinderPressure, cylinder.CylinderPressure);
            Assert.Equal(expectedInitialPressurisedVolume, cylinder.InitialPressurisedVolume);
            Assert.NotNull(cylinder.GasMixture);
            Assert.NotNull(cylinder.GasManagement);
        }

        private Mock<ICylinderController> StubCylinderController()
        {
            var stubCylinderController = new Mock<ICylinderController>();
            stubCylinderController.Setup(cc => cc.CalculateInitialPressurisedVolume(cylinderVolume, cylinderPressure)).Returns(expectedInitialPressurisedVolume);
            return stubCylinderController;
        }
    }
}