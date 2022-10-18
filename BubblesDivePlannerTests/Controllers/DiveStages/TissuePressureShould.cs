using BubblesDivePlanner.DiveStages;
using Xunit;

namespace BubblesDivePlannerTests.Controllers.DiveStages
{
    public class TissuePressureShould
    {
        [Fact]
        public void RunTissuePressureStages()
        {
            //Arrange
            var diveModel = TestFixture.FixtureDiveModel;
            var diveStep = TestFixture.FixtureDiveStep;
            diveModel.DiveProfile.OxygenPressureAtDepth = TestFixture.ExpectedOxygenPressureAtDepth;
            diveModel.DiveProfile.HeliumPressureAtDepth = TestFixture.ExpectedHeliumPressureAtDepth;
            diveModel.DiveProfile.NitrogenPressureAtDepth = TestFixture.ExpectedNitrogenPressureAtDepth;
            var diveStage = new TissuePressure(diveModel, diveStep);

            //Act
            diveStage.RunDiveStage();

            //Assert
            Assert.Equal(TestFixture.ExpectedNitrogenTissuePressures, diveModel.DiveProfile.NitrogenTissuePressures);
            Assert.Equal(TestFixture.ExpectedHeliumTissuePressures, diveModel.DiveProfile.HeliumTissuePressures);
            Assert.Equal(TestFixture.ExpectedTotalTissuePressures, diveModel.DiveProfile.TotalTissuePressures);
        }
    }
}