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
            DefaultValues(compartmentCount);
        }

        public double[] NitrogenTissuePressures { get; set; }
        public double[] HeliumTissuePressures { get; set; }
        public double[] TotalTissuePressures { get; set; }
        public double[] MaxSurfacePressures { get; set; }
        public double[] ToleratedAmbientPressures { get; set; }
        public double[] AValues { get; set; }
        public double[] BValues { get; set; }
        public double[] CompartmentLoads { get; set; }
        public double OxygenPressureAtDepth { get; set; }
        public double HeliumPressureAtDepth { get; set; }
        public double NitrogenPressureAtDepth { get; set; }

        private void DefaultValues(byte compartmentCount)
        {
            for (int compartment = 0; compartment < compartmentCount; compartment++)
            {
                NitrogenTissuePressures[compartment] = 0.79;
                TotalTissuePressures[compartment] = 0.79;
            }
        }

        public void UpdateGasMixtureUnderPressure(double oxygenPressureAtDepth, double heliumPressureAtDepth, double nitrogenPressureAtDepth)
        {
            OxygenPressureAtDepth = oxygenPressureAtDepth;
            HeliumPressureAtDepth = heliumPressureAtDepth;
            NitrogenPressureAtDepth = nitrogenPressureAtDepth;
        }
    }
}