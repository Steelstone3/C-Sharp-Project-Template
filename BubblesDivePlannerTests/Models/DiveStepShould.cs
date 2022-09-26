using BubblesDivePlanner.Models;
using Xunit;

namespace Name
{
    public class DiveStepShould
    {
        private readonly byte depth = 50;
        private readonly byte time = 10;

        [Fact]
        public void ContainsDepth()
        {
            IDiveStep diveStep = new DiveStep(depth, time);

            Assert.Equal(depth, diveStep.Depth);
        }

        [Fact]
        public void ContainsTime()
        {
            IDiveStep diveStep = new DiveStep(depth, time);

            Assert.Equal(time, diveStep.Time);
        }
    }
}