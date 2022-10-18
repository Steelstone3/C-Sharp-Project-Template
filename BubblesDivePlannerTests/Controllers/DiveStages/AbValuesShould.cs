using BubblesDivePlanner.Controllers.DiveStages;
using BubblesDivePlanner.Models.DiveModels;
using Xunit;

namespace BubblesDivePlannerTests.DiveStages
{
    public class AbValuesShould
    {

        [Fact]
        public void RunAbValuesStage()
        {
            //Arrange
            var diveModel = TestFixture.FixtureDiveModel;
            IDiveStageCommand diveStage = new AbValues(diveModel);

            //Act
            diveStage.RunDiveStage();

            //Assert
            Assert.Equal(TestFixture.ExpectedAValues, diveModel.DiveProfile.AValues);
            Assert.Equal(TestFixture.ExpectedBValues, diveModel.DiveProfile.BValues);
        }
    }
}