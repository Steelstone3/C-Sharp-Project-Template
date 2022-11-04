using BubblesDivePlanner.Models;
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
        private readonly ushort remainingGas = 2400;
        private readonly Mock<IGasMixture> dummyGasMixture = new();
        private ICylinder cylinder;

        public CylinderShould()
        {
            cylinder = new Cylinder(cylinderVolume, cylinderPressure, dummyGasMixture.Object, surfaceAirConsumptionRate);
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
        public void ContainsGasMixture()
        {
            Assert.NotNull(cylinder.GasMixture);
        }

        [Fact]
        public void ContainsRemainingGas()
        {
            Assert.Equal(remainingGas, cylinder.RemainingGas);
        }

        [Fact]
        public void ContainsGasUsed()
        {
            Assert.Equal(0, cylinder.UsedGas);
        }

        [Fact]
        public void ContainsSurfaceAirConsumptionRate()
        {
            Assert.Equal(surfaceAirConsumptionRate, cylinder.SurfaceAirConsumptionRate);
        }

        [Theory]
        [InlineData(12, 200, 2400)]
        [InlineData(24, 200, 4800)]
        [InlineData(12, 100, 1200)]
        [InlineData(12, 50, 600)]
        [InlineData(0, 200, 600)]
        [InlineData(12, 0, 600)]
        [InlineData(0, 0, 150)]
        [InlineData(31, 0, 1500)]
        [InlineData(0, 301, 900)]
        [InlineData(31, 301, 9000)]
        public void CalculateInitialPressurisedVolume(byte cylinderVolume, ushort cylinderPressure, ushort expectedInitialPressurisedVolume)
        {
            cylinder = new Cylinder(cylinderVolume, cylinderPressure, dummyGasMixture.Object, surfaceAirConsumptionRate);

            Assert.Equal(expectedInitialPressurisedVolume, cylinder.InitialPressurisedVolume);
        }

        [Theory]
        [InlineData(12, 200, 12, 50, 10, 720, 1680)]
        [InlineData(12, 200, 24, 50, 10, 1440, 960)]
        [InlineData(12, 200, 12, 100, 10, 1320, 1080)]
        [InlineData(12, 200, 12, 50, 20, 1440, 960)]
        [InlineData(12, 200, 0, 50, 10, 180, 2220)]
        [InlineData(12, 200, 12, 0, 10, 120, 2280)]
        [InlineData(12, 200, 12, 50, 0, 72, 2328)]
        [InlineData(12, 200, 12, 0, 0, 12, 2388)]
        [InlineData(12, 200, 0, 0, 0, 3, 2397)]
        public void CalculateGasUsage(byte cylinderVolume, ushort cylinderPressure, byte surfaceAirConsumptionRate, byte depth, byte time, ushort expectedUsedGas, ushort expectedRemainingGas)
        {
            cylinder = new Cylinder(cylinderVolume, cylinderPressure, dummyGasMixture.Object, surfaceAirConsumptionRate);
            IDiveStep diveStep = new DiveStep(depth, time);

            cylinder.UpdateCylinderGasConsumption(diveStep);

            Assert.Equal(expectedUsedGas, cylinder.UsedGas);
            Assert.Equal(expectedRemainingGas, cylinder.RemainingGas);
        }
    }
}