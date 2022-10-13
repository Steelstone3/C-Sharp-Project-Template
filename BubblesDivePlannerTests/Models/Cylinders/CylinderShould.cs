using BubblesDivePlanner.Models.Cylinders;
using Moq;
using Xunit;

namespace BubblesDivePlannerTests.Models.Cylinders
{
    public class CylinderShould
    {
        private readonly byte cylinderVolume = 12;
        private readonly ushort cylinderPressure = 200;
        private readonly byte surfaceAirConsumptionRate = 12;
        private readonly Mock<IGasMixture> dummyGasMixture = new();
        private ICylinder cylinder;

        public CylinderShould()
        {
            cylinder = new Cylinder( cylinderVolume, cylinderPressure, dummyGasMixture.Object, surfaceAirConsumptionRate);
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

        [Theory]
        [InlineData(12, 200, 2400)]
        [InlineData(24, 200, 4800)]
        [InlineData(12, 400, 4800)]
        [InlineData(0, 200, 0)]
        [InlineData(12, 0, 0)]
        [InlineData(0, 0, 0)]
        public void CalculateInitialPressurisedVolume(byte cylinderVolume, ushort cylinderPressure, ushort expectedInitialPressurisedVolume)
        {
            cylinder = new Cylinder( cylinderVolume, cylinderPressure, dummyGasMixture.Object, surfaceAirConsumptionRate);

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
    }
}