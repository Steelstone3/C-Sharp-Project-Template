using System.Collections.Generic;
using BubblesDivePlanner.Models;
using BubblesDivePlanner.Models.Cylinders;
using BubblesDivePlanner.Models.DiveModels;
using Moq;
using Xunit;

namespace BubblesDivePlannerTests.Models
{
    public class DivePlanShould
    {
        private Mock<IDiveModel> dummyDiveModel = new();
        private Mock<IList<ICylinder>> dummyCylinders = new();

        [Fact]
        public void ConstructADivePlan()
        {
            IDivePlan divePlan = new DivePlan(dummyDiveModel.Object, dummyCylinders.Object);

            Assert.NotNull(divePlan.DiveModel);
            Assert.NotNull(divePlan.Cylinders);
            Assert.Null(divePlan.DiveStep);
        }

        [Fact]
        public void UpdateDiveStep()
        {
            Mock<IDiveStep> dummyDiveStep = new();
            IDivePlan divePlan = new DivePlan(dummyDiveModel.Object, dummyCylinders.Object);
            divePlan.UpdateDiveStep(dummyDiveStep.Object);

            Assert.NotNull(divePlan.DiveModel);
            Assert.NotNull(divePlan.Cylinders);
            Assert.NotNull(divePlan.DiveStep);
        }
    }
}