using BubblesDivePlanner.Models;
using BubblesDivePlanner.Models.Cylinders;
using BubblesDivePlanner.Models.DiveModels;
using Moq;

namespace BubblesDivePlannerTests.TestFixtures
{
    public static class TestFixture
    {
        private const byte COMPARTMENT_COUNT = 16;

        public static IDiveModel FixtureDiveModel => new Zhl16Buhlmann();

        public static DiveStep FixtureDiveStep => new(50, 10);

        public static Cylinder FixtureSelectedCylinder => new(
        12,
        200,
        new GasMixture(21, 0),
        12
        );

        public static double ExpectedPressureOxygen => 1.26;
        public static double ExpectedPressureNitrogen => 4.74;
        public static double ExpectedPressureHelium => 0;
        public static double[] ExpectedAValues => new double[COMPARTMENT_COUNT] { 1.2559, 1.0000, 0.8618, 0.7562, 0.6667, 0.5600, 0.4947, 0.4500, 0.4187, 0.3798, 0.3497, 0.3223, 0.2850, 0.2737, 0.2523, 0.2327 };
        public static double[] ExpectedBValues => new double[COMPARTMENT_COUNT] { 0.5050, 0.6514, 0.7222, 0.7825, 0.8126, 0.8434, 0.8693, 0.8910, 0.9092, 0.9222, 0.9319, 0.9403, 0.9477, 0.9544, 0.9602, 0.9653 };
        public static double[] ExpectedMaxSurfacePressures => new double[COMPARTMENT_COUNT] { 3.2361, 2.5352, 2.2465, 2.0342, 1.8973, 1.7457, 1.6451, 1.5723, 1.5186, 1.4642, 1.4228, 1.3858, 1.3402, 1.3215, 1.2937, 1.2686 };
        public static double[] ExpectedNitrogenTissuePressures => new double[COMPARTMENT_COUNT] { 4.0417, 3.0792, 2.4713, 2.0243, 1.6843, 1.4439, 1.2634, 1.13, 1.0334, 0.9731, 0.9337, 0.9029, 0.8788, 0.8596, 0.8446, 0.8329 };
        public static double[] ExpectedHeliumTissuePressures => new double[COMPARTMENT_COUNT] { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
        public static double[] ExpectedTotalTissuePressures => new double[COMPARTMENT_COUNT] { 4.0417, 3.0792, 2.4713, 2.0243, 1.6843, 1.4439, 1.2634, 1.13, 1.0334, 0.9731, 0.9337, 0.9029, 0.8788, 0.8596, 0.8446, 0.8329 };
        public static double[] ExpectedToleratedAmbientPressures => new double[COMPARTMENT_COUNT] { 1.4068, 1.3544, 1.1624, 0.9923, 0.8269, 0.7455, 0.6682, 0.6059, 0.5589, 0.5471, 0.5442, 0.5459, 0.5627, 0.5592, 0.5687, 0.5794 };
        public static double[] ExpectedCompartmentLoads => new double[COMPARTMENT_COUNT] { 124.89, 121.46, 110.01, 99.51, 88.77, 82.71, 76.8, 71.87, 68.05, 66.46, 65.62, 65.15, 65.57, 65.05, 65.29, 65.66 };
    }
}