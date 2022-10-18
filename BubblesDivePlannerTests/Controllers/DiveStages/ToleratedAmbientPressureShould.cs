using BubblesDivePlanner.DiveStages;
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
            diveModel.DiveProfile.AValues = TestFixture.ExpectedAValues;
            diveModel.DiveProfile.BValues = TestFixture.ExpectedBValues;
            diveModel.DiveProfile.TotalTissuePressures = TestFixture.ExpectedTotalTissuePressures;
            var diveStage = new ToleratedAmbientPressure(diveModel);

            //Act
            diveStage.RunDiveStage();

            //Assert
            Assert.Equal(TestFixture.ExpectedToleratedAmbientPressures, diveModel.DiveProfile.ToleratedAmbientPressures);
        }
    }
}