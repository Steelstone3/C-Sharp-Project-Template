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
        private readonly Mock<IDiveModel> dummyDiveModel = new();
        private readonly Mock<IList<ICylinder>> dummyCylinders = new();

        [Fact]
        public void ContainADiveModel()
        {
            IDivePlan divePlan = new DivePlan(dummyDiveModel.Object, dummyCylinders.Object);

            Assert.NotNull(divePlan.DiveModel);
        }

        [Fact]
        public void ContainCylinders()
        {
            IDivePlan divePlan = new DivePlan(dummyDiveModel.Object, dummyCylinders.Object);

            Assert.NotNull(divePlan.Cylinders);
        }

        [Fact]
        public void ContainADiveStep() {
            IDivePlan divePlan = new DivePlan(dummyDiveModel.Object, dummyCylinders.Object);

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