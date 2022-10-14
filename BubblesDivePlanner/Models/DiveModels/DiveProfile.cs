namespace BubblesDivePlanner.Models.DiveModels
{
    public class DiveProfile : IDiveProfile
    {
        public DiveProfile(byte compartmentCount)
        {
            MaxSurfacePressures = new double[compartmentCount];
            TissuePressuresNitrogen = new double[compartmentCount];
            TissuePressuresHelium = new double[compartmentCount];
            TissuePressuresTotal = new double[compartmentCount];
            ToleratedAmbientPressures = new double[compartmentCount];
            AValues = new double[compartmentCount];
            BValues = new double[compartmentCount];
            CompartmentLoads = new double[compartmentCount];
        }

        public double[] MaxSurfacePressures
        {
            get; private set;
        }

        public double[] TissuePressuresNitrogen
        {
            get; private set;
        }

        public double[] TissuePressuresHelium
        {
            get; private set;
        }

        public double[] TissuePressuresTotal
        {
            get; private set;
        }

        public double[] ToleratedAmbientPressures
        {
            get; private set;
        }

        public double[] AValues
        {
            get; private set;
        }

        public double[] BValues
        {
            get; private set;
        }

        public double[] CompartmentLoads
        {
            get; private set;
        }

        public double PressureOxygen
        {
            get; private set;
        } = 0.0;

        public double PressureHelium
        {
            get; private set;
        } = 0.0;

        public double PressureNitrogen
        {
            get; private set;
        } = 0.0;
    }
}