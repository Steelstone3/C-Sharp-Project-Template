using BubblesDivePlanner.Controllers.DiveStages;
using BubblesDivePlanner.Models.DiveModels;
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
            diveModel.DiveProfile.UpdateDiveProfile(new DiveProfile
            (
                null,
                null,
                null,
                TestFixture.DefaultList,
                null,
                TestFixture.ExpectedAValues, 
                TestFixture.ExpectedBValues,
                null,
                0,
                0,
                0
            ));
            var diveStage = new MaximumSurfacePressure(diveModel);

            //Act
            diveStage.RunDiveStage();

            //Assert
            Assert.Equal(TestFixture.ExpectedMaxSurfacePressures, diveModel.DiveProfile.MaxSurfacePressures);
        }
    }
}