// using BubblesDivePlanner.Controllers.DiveStages;
// using BubblesDivePlanner.DiveStages;
// using BubblesDivePlanner.Models.DiveModels;
// using BubblesDivePlannerTests.TestFixtures;
// using Moq;
// using Xunit;

// namespace BubblesDivePlannerTests.DiveStages
// {
//     public class MaximumSurfacePressureCommandShould
//     {
        
//         [Fact]
//         public void RunMaximumSurfacePressureStage()
//         {
//             //Arrange
//             var diveModel = new Zhl16Buhlmann();
//             diveModel.DiveProfile = ExpectedDiveProfile();
//             // diveModel.DiveProfile = DivePlannerApplicationTestFixture.GetDiveProfileResultFromFirstRun;
//             // var aValues = diveModel.DiveProfile.AValues;
//             // var bValues = diveModel.DiveProfile.BValues;
//             // diveModel.DiveProfile.AValues = aValues;
//             // diveModel.DiveProfile.BValues = bValues;
//             var expectedMaxSurfacePresureResult = diveModel.DiveProfile.MaxSurfacePressures;

//             var diveStage = new MaximumSurfacePressureCommand(diveModel);

//             //Act
//             diveStage.RunDiveStage();

//             //Assert
//             Assert.Equal(expectedMaxSurfacePresureResult, diveModel.DiveProfile.MaxSurfacePressures);
//         }

//         private IDiveProfile ExpectedDiveProfile()
//         {
//             var diveProfile = new Mock<IDiveProfile>();
//             diveProfile.Setup(dp => dp.MaxSurfacePressures).Returns(new double[1]);

//             return diveProfile.Object;
//         }
//     }
// }