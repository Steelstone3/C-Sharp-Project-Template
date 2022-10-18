// using Xunit;

// namespace BubblesDivePlannerTests.Controllers.DiveStages
// {
//     public class ToleratedAmbientPressureShould
//     {
//         [Fact]
//         public void RunToleratedAmbientPressureStage()
//         {
//             //Arrange
//             var diveModel = DivePlannerApplicationTestFixture.GetDiveModel;
//             diveModel.DiveProfile = DivePlannerApplicationTestFixture.GetDiveProfileResultFromFirstRun;
//             var toleratedAmbientPressuresResult = DivePlannerApplicationTestFixture.GetDiveProfileResultFromFirstRun.ToleratedAmbientPressures;

//             var diveStage = new ToleratedAmbientPressureCommand(diveModel);

//             //Act
//             diveStage.RunDiveStage();

//             //Assert
//             Assert.Equal(toleratedAmbientPressuresResult, diveModel.DiveProfile.ToleratedAmbientPressures);
//         }
//     }
// }