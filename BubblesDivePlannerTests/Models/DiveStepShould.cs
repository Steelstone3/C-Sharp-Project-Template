using BubblesDivePlanner.Models;
using Xunit;

namespace Name
{
    public class DiveStepShould
    {
        [Theory]
        [InlineData(50, 50)]
        [InlineData(60, 60)]
        [InlineData(100, 100)]
        [InlineData(101, 100)]
        [InlineData(200, 100)]
        public void ContainsDepth(byte depth, byte expectedDepth)
        {
            byte time = 10;

            IDiveStep diveStep = new DiveStep(depth, time);

            Assert.Equal(expectedDepth, diveStep.Depth);
        }

        [Theory]
        [InlineData(50,  50)]
        [InlineData(60, 60)]
        [InlineData(61, 60)]
        [InlineData(70, 60)]
        public void ContainsTime(byte time, byte expectedTime)
        {
            byte depth = 50;

            IDiveStep diveStep = new DiveStep(depth, time);

            Assert.Equal(expectedTime, diveStep.Time);
        }
    }
}