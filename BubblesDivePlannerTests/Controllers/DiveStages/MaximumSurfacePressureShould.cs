using BubblesDivePlanner.Controllers.DiveStages;
using Xunit;

namespace BubblesDivePlannerTests.Controllers.DiveStages
{
    public class MaximumSurfacePressureShould
    {
        [Fact]
        public void RunMaximumSurfacePressureStage()
        {
            //Arrange
            var diveModel = TestFixture.FixtureDiveModel;
            diveModel.DiveProfile.AValues = TestFixture.ExpectedAValues;
            diveModel.DiveProfile.BValues = TestFixture.ExpectedBValues;
            var diveStage = new MaximumSurfacePressure(diveModel);

            //Act
            diveStage.RunDiveStage();

            //Assert
            Assert.Equal(TestFixture.ExpectedMaxSurfacePressures, diveModel.DiveProfile.MaxSurfacePressures);
        }
    }
}