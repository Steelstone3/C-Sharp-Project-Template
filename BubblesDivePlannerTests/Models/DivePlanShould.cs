using System.Collections.Generic;
using System.Runtime.InteropServices;
using BubblesDivePlanner.Models;
using BubblesDivePlanner.Models.Cylinders;
using BubblesDivePlanner.Models.DiveModels;
using Xunit;

namespace BubblesDivePlannerTests.Models
{
    public class DivePlanShould
    {
        private const string EXPECTED_DIVE_PLAN_JSON_UNIX = "{\n  \"DiveModel\": {\n    \"Name\": \"Zhl16-B Model\",\n    \"CompartmentCount\": 16,\n    \"NitrogenHalfTimes\": [\n      4.0,\n      8.0,\n      12.5,\n      18.5,\n      27.0,\n      38.3,\n      54.3,\n      77.0,\n      109.0,\n      146.0,\n      187.0,\n      239.0,\n      305.0,\n      390.0,\n      498.0,\n      635.0\n    ],\n    \"HeliumHalfTimes\": [\n      1.51,\n      3.02,\n      4.72,\n      6.99,\n      10.21,\n      14.48,\n      20.53,\n      29.11,\n      41.2,\n      55.19,\n      70.69,\n      90.34,\n      115.29,\n      147.42,\n      188.24,\n      240.03\n    ],\n    \"AValuesNitrogen\": [\n      1.2559,\n      1.0,\n      0.8618,\n      0.7562,\n      0.6667,\n      0.56,\n      0.4947,\n      0.45,\n      0.4187,\n      0.3798,\n      0.3497,\n      0.3223,\n      0.285,\n      0.2737,\n      0.2523,\n      0.2327\n    ],\n    \"BValuesNitrogen\": [\n      0.505,\n      0.6514,\n      0.7222,\n      0.7825,\n      0.8126,\n      0.8434,\n      0.8693,\n      0.891,\n      0.9092,\n      0.9222,\n      0.9319,\n      0.9403,\n      0.9477,\n      0.9544,\n      0.9602,\n      0.9653\n    ],\n    \"AValuesHelium\": [\n      1.7424,\n      1.383,\n      1.1919,\n      1.0458,\n      0.922,\n      0.8205,\n      0.7305,\n      0.6502,\n      0.595,\n      0.5545,\n      0.5333,\n      0.5189,\n      0.5181,\n      0.5176,\n      0.5172,\n      0.5119\n    ],\n    \"BValuesHelium\": [\n      0.4245,\n      0.5747,\n      0.6527,\n      0.7223,\n      0.7582,\n      0.7957,\n      0.8279,\n      0.8553,\n      0.8757,\n      0.8903,\n      0.8997,\n      0.9073,\n      0.9122,\n      0.9171,\n      0.9217,\n      0.9267\n    ],\n    \"DiveProfile\": {\n      \"NitrogenTissuePressures\": [\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79\n      ],\n      \"HeliumTissuePressures\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"TotalTissuePressures\": [\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79\n      ],\n      \"MaxSurfacePressures\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"ToleratedAmbientPressures\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"AValues\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"BValues\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"CompartmentLoads\": [\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0,\n        0.0\n      ],\n      \"OxygenPressureAtDepth\": 0.0,\n      \"HeliumPressureAtDepth\": 0.0,\n      \"NitrogenPressureAtDepth\": 0.0\n    }\n  },\n  \"Cylinders\": [\n    {\n      \"CylinderVolume\": 12,\n      \"CylinderPressure\": 200,\n      \"InitialPressurisedVolume\": 2400,\n      \"RemainingGas\": 2400,\n      \"UsedGas\": 0,\n      \"SurfaceAirConsumptionRate\": 12,\n      \"GasMixture\": {\n        \"Oxygen\": 21,\n        \"Helium\": 10,\n        \"Nitrogen\": 69\n      }\n    }\n  ],\n  \"DiveStep\": {\n    \"Depth\": 50,\n    \"Time\": 10\n  }\n}";
        private const string EXPECTED_RAN_DIVE_PLAN_JSON_UNIX = "{\n  \"DiveModel\": {\n    \"Name\": \"Zhl16-B Model\",\n    \"CompartmentCount\": 16,\n    \"NitrogenHalfTimes\": [\n      4.0,\n      8.0,\n      12.5,\n      18.5,\n      27.0,\n      38.3,\n      54.3,\n      77.0,\n      109.0,\n      146.0,\n      187.0,\n      239.0,\n      305.0,\n      390.0,\n      498.0,\n      635.0\n    ],\n    \"HeliumHalfTimes\": [\n      1.51,\n      3.02,\n      4.72,\n      6.99,\n      10.21,\n      14.48,\n      20.53,\n      29.11,\n      41.2,\n      55.19,\n      70.69,\n      90.34,\n      115.29,\n      147.42,\n      188.24,\n      240.03\n    ],\n    \"AValuesNitrogen\": [\n      1.2559,\n      1.0,\n      0.8618,\n      0.7562,\n      0.6667,\n      0.56,\n      0.4947,\n      0.45,\n      0.4187,\n      0.3798,\n      0.3497,\n      0.3223,\n      0.285,\n      0.2737,\n      0.2523,\n      0.2327\n    ],\n    \"BValuesNitrogen\": [\n      0.505,\n      0.6514,\n      0.7222,\n      0.7825,\n      0.8126,\n      0.8434,\n      0.8693,\n      0.891,\n      0.9092,\n      0.9222,\n      0.9319,\n      0.9403,\n      0.9477,\n      0.9544,\n      0.9602,\n      0.9653\n    ],\n    \"AValuesHelium\": [\n      1.7424,\n      1.383,\n      1.1919,\n      1.0458,\n      0.922,\n      0.8205,\n      0.7305,\n      0.6502,\n      0.595,\n      0.5545,\n      0.5333,\n      0.5189,\n      0.5181,\n      0.5176,\n      0.5172,\n      0.5119\n    ],\n    \"BValuesHelium\": [\n      0.4245,\n      0.5747,\n      0.6527,\n      0.7223,\n      0.7582,\n      0.7957,\n      0.8279,\n      0.8553,\n      0.8757,\n      0.8903,\n      0.8997,\n      0.9073,\n      0.9122,\n      0.9171,\n      0.9217,\n      0.9267\n    ],\n    \"DiveProfile\": {\n      \"NitrogenTissuePressures\": [\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79,\n        0.79\n      ],\n      \"HeliumTissuePressures\": [\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5\n      ],\n      \"TotalTissuePressures\": [\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5\n      ],\n      \"MaxSurfacePressures\": [\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5\n      ],\n      \"ToleratedAmbientPressures\": [\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5\n      ],\n      \"AValues\": [\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5\n      ],\n      \"BValues\": [\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5\n      ],\n      \"CompartmentLoads\": [\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5,\n        12.5\n      ],\n      \"OxygenPressureAtDepth\": 12.5,\n      \"HeliumPressureAtDepth\": 12.5,\n      \"NitrogenPressureAtDepth\": 12.5\n    }\n  },\n  \"Cylinders\": [\n    {\n      \"CylinderVolume\": 12,\n      \"CylinderPressure\": 200,\n      \"InitialPressurisedVolume\": 2400,\n      \"RemainingGas\": 2400,\n      \"UsedGas\": 0,\n      \"SurfaceAirConsumptionRate\": 12,\n      \"GasMixture\": {\n        \"Oxygen\": 21,\n        \"Helium\": 10,\n        \"Nitrogen\": 69\n      }\n    }\n  ],\n  \"DiveStep\": {\n    \"Depth\": 50,\n    \"Time\": 10\n  }\n}";
        private const string EXPECTED_DIVE_PLAN_JSON_WINDOWS = "{\r\n  \"DiveModel\": {\r\n    \"Name\": \"Zhl16-B Model\",\r\n    \"CompartmentCount\": 16,\r\n    \"NitrogenHalfTimes\": [\r\n      4.0,\r\n      8.0,\r\n      12.5,\r\n      18.5,\r\n      27.0,\r\n      38.3,\r\n      54.3,\r\n      77.0,\r\n      109.0,\r\n      146.0,\r\n      187.0,\r\n      239.0,\r\n      305.0,\r\n      390.0,\r\n      498.0,\r\n      635.0\r\n    ],\r\n    \"HeliumHalfTimes\": [\r\n      1.51,\r\n      3.02,\r\n      4.72,\r\n      6.99,\r\n      10.21,\r\n      14.48,\r\n      20.53,\r\n      29.11,\r\n      41.2,\r\n      55.19,\r\n      70.69,\r\n      90.34,\r\n      115.29,\r\n      147.42,\r\n      188.24,\r\n      240.03\r\n    ],\r\n    \"AValuesNitrogen\": [\r\n      1.2559,\r\n      1.0,\r\n      0.8618,\r\n      0.7562,\r\n      0.6667,\r\n      0.56,\r\n      0.4947,\r\n      0.45,\r\n      0.4187,\r\n      0.3798,\r\n      0.3497,\r\n      0.3223,\r\n      0.285,\r\n      0.2737,\r\n      0.2523,\r\n      0.2327\r\n    ],\r\n    \"BValuesNitrogen\": [\r\n      0.505,\r\n      0.6514,\r\n      0.7222,\r\n      0.7825,\r\n      0.8126,\r\n      0.8434,\r\n      0.8693,\r\n      0.891,\r\n      0.9092,\r\n      0.9222,\r\n      0.9319,\r\n      0.9403,\r\n      0.9477,\r\n      0.9544,\r\n      0.9602,\r\n      0.9653\r\n    ],\r\n    \"AValuesHelium\": [\r\n      1.7424,\r\n      1.383,\r\n      1.1919,\r\n      1.0458,\r\n      0.922,\r\n      0.8205,\r\n      0.7305,\r\n      0.6502,\r\n      0.595,\r\n      0.5545,\r\n      0.5333,\r\n      0.5189,\r\n      0.5181,\r\n      0.5176,\r\n      0.5172,\r\n      0.5119\r\n    ],\r\n    \"BValuesHelium\": [\r\n      0.4245,\r\n      0.5747,\r\n      0.6527,\r\n      0.7223,\r\n      0.7582,\r\n      0.7957,\r\n      0.8279,\r\n      0.8553,\r\n      0.8757,\r\n      0.8903,\r\n      0.8997,\r\n      0.9073,\r\n      0.9122,\r\n      0.9171,\r\n      0.9217,\r\n      0.9267\r\n    ],\r\n    \"DiveProfile\": {\r\n      \"NitrogenTissuePressures\": [\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79\r\n      ],\r\n      \"HeliumTissuePressures\": [\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0\r\n      ],\r\n      \"TotalTissuePressures\": [\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79,\r\n        0.79\r\n      ],\r\n      \"MaxSurfacePressures\": [\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0\r\n      ],\r\n      \"ToleratedAmbientPressures\": [\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0\r\n      ],\r\n      \"AValues\": [\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0\r\n      ],\r\n      \"BValues\": [\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0\r\n      ],\r\n      \"CompartmentLoads\": [\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0,\r\n        0.0\r\n      ],\r\n      \"OxygenPressureAtDepth\": 0.0,\r\n      \"HeliumPressureAtDepth\": 0.0,\r\n      \"NitrogenPressureAtDepth\": 0.0\r\n    }\r\n  },\r\n  \"Cylinders\": [\r\n    {\r\n      \"CylinderVolume\": 12,\r\n      \"CylinderPressure\": 200,\r\n      \"InitialPressurisedVolume\": 2400,\r\n      \"RemainingGas\": 2400,\r\n      \"UsedGas\": 0,\r\n      \"SurfaceAirConsumptionRate\": 12,\r\n      \"GasMixture\": {\r\n        \"Oxygen\": 21,\r\n        \"Helium\": 10,\r\n        \"Nitrogen\": 69\r\n      }\r\n    }\r\n  ],\r\n  \"DiveStep\": {\r\n    \"Depth\": 50,\r\n    \"Time\": 10\r\n  }\r\n}";
        private readonly IDiveModel diveModel = new Zhl16Buhlmann();
        private readonly ICylinder cylinder = new Cylinder(12, 200, new GasMixture(21, 10), 12);
        private readonly List<ICylinder> cylinders = new();
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
            Assert.Equal(diveStep, divePlan.DiveStep);
        }

        [SkippableFact]
        public void SerialiseUnix()
        {
            Skip.If(RuntimeInformation.IsOSPlatform(OSPlatform.Windows));
            divePlan.UpdateDiveStep(new DiveStep(50, 10));

            var divePlanJson = divePlan.Serialise();

            Assert.Equal(EXPECTED_DIVE_PLAN_JSON_UNIX, divePlanJson);
        }

        [SkippableFact]
        public void SerialiseWindows()
        {
            Skip.IfNot(RuntimeInformation.IsOSPlatform(OSPlatform.Windows));
            divePlan.UpdateDiveStep(new DiveStep(50, 10));

            var divePlanJson = divePlan.Serialise();

            Assert.Equal(EXPECTED_DIVE_PLAN_JSON_WINDOWS, divePlanJson);
        }

        [Theory]
        [InlineData(EXPECTED_DIVE_PLAN_JSON_UNIX)]
        [InlineData(EXPECTED_DIVE_PLAN_JSON_WINDOWS)]
        public void Deserialise(string expectedJsonString)
        {
            IDivePlan actualDivePlan = new DivePlan(null, null);
            
            actualDivePlan.Deserialise(expectedJsonString);

            AssertDiveModel(divePlan.DiveModel, actualDivePlan.DiveModel);
            AssertCylinders(divePlan.Cylinders, actualDivePlan.Cylinders);
        }

        [Fact(Skip="Doesn't populate values")]
        public void DeserialiseRanDivePlan()
        {
            var defaultValue = 12.5;
            var defaultList = new double[16]
            {
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
                defaultValue,
            };
            divePlan.DiveModel.DiveProfile.UpdateDiveProfile(new DiveProfile
            (
                defaultList,
                defaultList,
                defaultList,
                defaultList,
                defaultList,
                defaultList,
                defaultList,
                defaultList,
                defaultValue,
                defaultValue,
                defaultValue
            ));
            IDivePlan actualDivePlan = new DivePlan(null, null);

            actualDivePlan.Deserialise(EXPECTED_RAN_DIVE_PLAN_JSON_UNIX);

            AssertDiveModel(divePlan.DiveModel, actualDivePlan.DiveModel);
            AssertCylinders(divePlan.Cylinders, actualDivePlan.Cylinders);
        }

        private static void AssertDiveModel(IDiveModel expected, IDiveModel actual)
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

        private static void AssertDiveProfile(IDiveProfile expected, IDiveProfile actual)
        {
            Assert.Equal(expected.OxygenPressureAtDepth, actual.OxygenPressureAtDepth);
            Assert.Equal(expected.HeliumPressureAtDepth, actual.HeliumPressureAtDepth);
            Assert.Equal(expected.NitrogenPressureAtDepth, actual.NitrogenPressureAtDepth);
            Assert.Equal(expected.NitrogenTissuePressures, actual.NitrogenTissuePressures);
            Assert.Equal(expected.HeliumTissuePressures, actual.HeliumTissuePressures);
            Assert.Equal(expected.TotalTissuePressures, actual.TotalTissuePressures);
            Assert.Equal(expected.AValues, actual.AValues);
            Assert.Equal(expected.BValues, actual.BValues);
            Assert.Equal(expected.MaxSurfacePressures, actual.MaxSurfacePressures);
            Assert.Equal(expected.ToleratedAmbientPressures, actual.ToleratedAmbientPressures);
            Assert.Equal(expected.CompartmentLoads, actual.CompartmentLoads);
        }

        private static void AssertCylinders(IList<ICylinder> expected, IList<ICylinder> actual)
        {
            Assert.NotEmpty(actual);

            for (int index = 0; index < expected.Count - 1; index++)
            {
                Assert.Equal(expected[index], actual[index]);
            }
        }
    }
}