namespace BubblesDivePlanner.Models.DiveModels
{
    public class DiveProfile : IDiveProfile
    {
        public DiveProfile(byte compartmentCount)
        {
            MaxSurfacePressures = new double[compartmentCount];
            NitrogenTissuePressures = new double[compartmentCount];
            HeliumTissuePressures = new double[compartmentCount];
            TotalTissuePressures = new double[compartmentCount];
            ToleratedAmbientPressures = new double[compartmentCount];
            AValues = new double[compartmentCount];
            BValues = new double[compartmentCount];
            CompartmentLoads = new double[compartmentCount];
        }

        public double[] NitrogenTissuePressures { get; private set; }
        public double[] HeliumTissuePressures { get; private set; }
        public double[] TotalTissuePressures { get; private set; }
        public double[] MaxSurfacePressures { get; private set; }
        public double[] ToleratedAmbientPressures { get; private set; }
        public double[] AValues { get; private set; }
        public double[] BValues { get; private set; }
        public double[] CompartmentLoads { get; private set; }
        public double OxygenPressureAtDepth { get; private set; }
        public double HeliumPressureAtDepth { get; private set; }
        public double NitrogenPressureAtDepth { get; private set; }

        public void UpdateGasMixtureUnderPressure(double oxygenPressureAtDepth, double heliumPressureAtDepth, double nitrogenPressureAtDepth)
        {
            OxygenPressureAtDepth = oxygenPressureAtDepth;
            HeliumPressureAtDepth = heliumPressureAtDepth;
            NitrogenPressureAtDepth = nitrogenPressureAtDepth;
        }
    }
}