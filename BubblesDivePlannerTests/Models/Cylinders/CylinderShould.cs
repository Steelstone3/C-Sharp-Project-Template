using BubblesDivePlanner.Models.Cylinders;
using Moq;
using Xunit;

namespace BubblesDivePlannerTests.Models.Cylinders
{
    public class CylinderShould
    {
        [Fact]
        public void ConstructACylinder()
        {
            byte cylinderVolume = 12;
            ushort cylinderPressure = 200;
            ushort expectedInitialPressurisedVolume = 2400;
            Mock<IGasMixture> dummyGasMixture = new();
            byte surfaceAirConsumptionRate = 12;
            ICylinder cylinder = new Cylinder(cylinderVolume, cylinderPressure, dummyGasMixture.Object, surfaceAirConsumptionRate);

            Assert.Equal(cylinderVolume, cylinder.CylinderVolume);
            Assert.Equal(cylinderPressure, cylinder.CylinderPressure);
            Assert.Equal(expectedInitialPressurisedVolume, cylinder.InitialPressurisedVolume);
            Assert.NotNull(cylinder.GasMixture);
            Assert.NotNull(cylinder.GasManagement);
        }
    }
}