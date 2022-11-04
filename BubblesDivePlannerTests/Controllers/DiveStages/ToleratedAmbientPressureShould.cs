using BubblesDivePlanner.DiveStages;
using BubblesDivePlanner.Models.DiveModels;
using Xunit;

namespace BubblesDivePlannerTests.Controllers.DiveStages
{
    public class ToleratedAmbientPressureShould
    {
        [Fact]
        public void RunToleratedAmbientPressureStage()
        {
            //Arrange
            var diveModel = TestFixture.FixtureDiveModel;
            diveModel.DiveProfile.UpdateDiveProfile(new DiveProfile
            (
                null,
                null,
                TestFixture.ExpectedTotalTissuePressures,
                null,
                TestFixture.DefaultList,
                TestFixture.ExpectedAValues,
                TestFixture.ExpectedBValues,
                null,
                0,
                0,
                0
            ));
            var diveStage = new ToleratedAmbientPressure(diveModel);

            //Act
            diveStage.RunDiveStage();

            //Assert
            Assert.Equal(TestFixture.ExpectedToleratedAmbientPressures, diveModel.DiveProfile.ToleratedAmbientPressures);
        }
    }
}