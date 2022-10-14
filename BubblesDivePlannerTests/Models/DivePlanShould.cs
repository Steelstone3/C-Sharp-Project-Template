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
        private readonly IDiveModel diveModel = new Zhl16Buhlmann();
        private readonly ICylinder cylinder = new Cylinder(12, 200, new GasMixture(21, 10), 12);
        private readonly List<ICylinder> cylinders = new();
        private readonly string expectedDivePlanJson = "{\n  \"DiveModel\": {\n    \"Name\": \"Zhl16-B Model\",\n    \"CompartmentCount\": 16,\n    \"NitrogenHalfTimes\": [\n      4.0,\n      8.0,\n      12.5,\n      18.5,\n      27.0,\n      38.3,\n      54.3,\n      77.0,\n      109.0,\n      146.0,\n      187.0,\n      239.0,\n      305.0,\n      390.0,\n      498.0,\n      635.0\n    ],\n    \"HeliumHalfTimes\": [\n      1.51,\n      3.02,\n      4.72,\n      6.99,\n      10.21,\n      14.48,\n      20.53,\n      29.11,\n      41.2,\n      55.19,\n      70.69,\n      90.34,\n      115.29,\n      147.42,\n      188.24,\n      240.03\n    ],\n    \"AValuesNitrogen\": [\n      1.2559,\n      1.0,\n      0.8618,\n      0.7562,\n      0.6667,\n      0.56,\n      0.4947,\n      0.45,\n      0.4187,\n      0.3798,\n      0.3497,\n      0.3223,\n      0.285,\n      0.2737,\n      0.2523,\n      0.2327\n    ],\n    \"BValuesNitrogen\": [\n      0.505,\n      0.6514,\n      0.7222,\n      0.7825,\n      0.8126,\n      0.8434,\n      0.8693,\n      0.891,\n      0.9092,\n      0.9222,\n      0.9319,\n      0.9403,\n      0.9477,\n      0.9544,\n      0.9602,\n      0.9653\n    ],\n    \"AValuesHelium\": [\n      1.7424,\n      1.383,\n      1.1919,\n      1.0458,\n      0.922,\n      0.8205,\n      0.7305,\n      0.6502,\n      0.595,\n      0.5545,\n      0.5333,\n      0.5189,\n      0.5181,\n      0.5176,\n      0.5172,\n      0.5119\n    ],\n    \"BValuesHelium\": [\n      0.4245,\n      0.5747,\n      0.6527,\n      0.7223,\n      0.7582,\n      0.7957,\n      0.8279,\n      0.8553,\n      0.8757,\n      0.8903,\n      0.8997,\n      0.9073,\n      0.9122,\n      0.9171,\n      0.9217,\n      0.9267\n    ],\n    \"DiveProfile\": {\n      \"MaxSurfacePressures\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"TissuePressuresNitrogen\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"TissuePressuresHelium\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"TissuePressuresTotal\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"ToleratedAmbientPressures\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"AValues\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"BValues\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"CompartmentLoads\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"PressureOxygen\": 0.0,\n      \"PressureHelium\": 0.0,\n      \"PressureNitrogen\": 0.0\n    }\n  },\n  \"Cylinders\": [\n    {\n      \"CylinderVolume\": 12,\n      \"CylinderPressure\": 200,\n      \"InitialPressurisedVolume\": 2400,\n      \"RemainingGas\": 2400,\n      \"UsedGas\": 0,\n      \"SurfaceAirConsumptionRate\": 12,\n      \"GasMixture\": {\n        \"Oxygen\": 21,\n        \"Helium\": 10,\n        \"Nitrogen\": 69\n      }\n    }\n  ],\n  \"DiveStep\": {\n    \"Depth\": 50,\n    \"Time\": 10\n  }\n}";
        private readonly IDivePlan divePlan;

        public DivePlanShould()
        {
            cylinders.Add(cylinder);
            divePlan = new DivePlan(diveModel, cylinders);
        }

        [Fact]
        public void ContainADiveModel()
        {
            IDivePlan divePlan = new DivePlan(diveModel, cylinders);

            Assert.NotNull(divePlan.DiveModel);
        }

        [Fact]
        public void ContainCylinders()
        {
            Assert.NotNull(divePlan.Cylinders);
        }

        [Fact]
        public void ContainADiveStep()
        {
            Assert.Null(divePlan.DiveStep);
        }

        [Fact]
        public void UpdateDiveStep()
        {
            byte depth = 50;
            byte time = 10;
            IDiveStep diveStep = new DiveStep(depth, time);

            divePlan.UpdateDiveStep(diveStep);

            Assert.NotNull(divePlan.DiveModel);
            Assert.NotNull(divePlan.Cylinders);
            Assert.NotNull(divePlan.DiveStep);
            Assert.Equal(depth, divePlan.DiveStep.Depth);
            Assert.Equal(time, divePlan.DiveStep.Time);
        }

        [Fact]
        public void Serialise()
        {
            divePlan.UpdateDiveStep(new DiveStep(50, 10));

            var divePlanJson = divePlan.Serialise();

            Assert.Equal(expectedDivePlanJson, divePlanJson);
        }

        [Fact(Skip="Doesn't work yet")]
        public void Deserialise()
        {
            var dummyDiveModel = new Mock<IDiveModel>();
            var dummyCylinders = new Mock<IList<ICylinder>>();
            IDivePlan actualDivePlan = new DivePlan(dummyDiveModel.Object, dummyCylinders.Object);

            actualDivePlan.Deserialise(expectedDivePlanJson);

            Assert.Equal(divePlan, actualDivePlan);
        }
    }
}