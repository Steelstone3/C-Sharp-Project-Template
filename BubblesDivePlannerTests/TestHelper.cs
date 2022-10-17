using System;
using System.Collections.Generic;
using BubblesDivePlanner.Models.Cylinders;
using BubblesDivePlanner.Models.DiveModels;
using Xunit;

namespace BubblesDivePlannerTests
{
    public static class TestHelper
    {
        public static void AssertDiveModel(IDiveModel expected, IDiveModel actual)
        {
            Assert.Equal(expected.Name, actual.Name);
            Assert.Equal(expected.CompartmentCount, actual.CompartmentCount);
            Assert.Equal(expected.NitrogenHalfTimes, actual.NitrogenHalfTimes);
            Assert.Equal(expected.HeliumHalfTimes, actual.HeliumHalfTimes);
            Assert.Equal(expected.AValuesNitrogen, actual.AValuesNitrogen);
            Assert.Equal(expected.BValuesNitrogen, actual.BValuesNitrogen);
            Assert.Equal(expected.AValuesHelium, actual.AValuesHelium);
            Assert.Equal(expected.BValuesHelium, actual.BValuesHelium);
            AssertDiveProfile(expected.DiveProfile, actual.DiveProfile);
        }

        public static void AssertCylinders(IList<ICylinder> expected, IList<ICylinder> actual)
        {
            Assert.NotEmpty(actual);
            
            for (int index = 0; index < expected.Count - 1; index++)
            {
                Assert.Equal(expected[index], actual[index]);
            }
        }

        private static void AssertDiveProfile(IDiveProfile expected, IDiveProfile actual)
        {
            Assert.Equal(expected.PressureOxygen, actual.PressureOxygen);
            Assert.Equal(expected.PressureHelium, actual.PressureHelium);
            Assert.Equal(expected.PressureNitrogen, actual.PressureNitrogen);
            Assert.Equal(expected.TissuePressuresNitrogen, actual.TissuePressuresNitrogen);
            Assert.Equal(expected.TissuePressuresHelium, actual.TissuePressuresHelium);
            Assert.Equal(expected.TissuePressuresTotal, actual.TissuePressuresTotal);
            Assert.Equal(expected.AValues, actual.AValues);
            Assert.Equal(expected.BValues, actual.BValues);
            Assert.Equal(expected.MaxSurfacePressures, actual.MaxSurfacePressures);
            Assert.Equal(expected.ToleratedAmbientPressures, actual.ToleratedAmbientPressures);
            Assert.Equal(expected.CompartmentLoads, actual.CompartmentLoads);
        }
    }
}