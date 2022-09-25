using System.Collections.Generic;
using BattleSimulator.Models.Mechs;
using Spectre.Console;
using System.Linq;
using BattleSimulator.Models.Weapons;

namespace BattleSimulator.Presenters
{
    public class Presenter : IPresenter
    {
        public void DisplayMech(IMech mechToDisplay)
        {
            //Fail early
            if (mechToDisplay == null)
            {
                return;
            }

            AnsiConsole.WriteLine($"Name: {mechToDisplay.Name} |");
            AnsiConsole.WriteLine($"Shields: {mechToDisplay.Shields} |");
            AnsiConsole.WriteLine($"Hull Integrity: {mechToDisplay.Hull} |");

            byte index = 0;
            foreach (var weapon in mechToDisplay.Weapons)
            {
                AnsiConsole.WriteLine($"Weapon: #{index}.{weapon.Name} |");
                index++;
            }

            AnsiConsole.WriteLine();
        }

        public IMech SelectMech(IList<IMech> mechs)
        {
            //Fail early
            if (!mechs.Any()) { return null; }

            /*TODO ideally you'd use the IMech type as the list for the "SelectionPrompt"
            
            Currently we are taking a list of mechs
            Converting them to strings based on the names
            Using those names in the selection prompt to be "displayable" to a user
            Making a selection based on the name
            Then finding the first mech object with that name the user selected
            This isn't always going to be the mech the user acutally targetted
            A solution is unique names for every mech
            The better solution is use the mech object in the "SelectionPrompt<IMech>"

            */
            var mechNames = mechs.Select(m => m.Name).ToArray();

            // Ask for the user's target mech
            var mech = AnsiConsole.Prompt(
                new SelectionPrompt<string>()
                    .Title("Select target [green]mech:[/]")
                    .PageSize(10)
                    .MoreChoicesText("[grey](Move up and down to reveal more mechs)[/]")
                    .AddChoices(mechNames));

            // Echo the mech to be fired out in the console
            AnsiConsole.WriteLine($"Destruction commencing for {mech}!");
            AnsiConsole.WriteLine();

            //TODO So by proxy of my above comment this is a hack!
            return mechs.FirstOrDefault(m => m.Name == mech);
        }

        public IWeapon SelectWeapon(IList<IWeapon> weapons)
        {
            //TODO read above comment where in this case it would only matter 
            //if the weapons had different power levels based on some sort of XP system or something
            var weaponNames = weapons.Select(m => m.Name).ToArray();

            // Ask for the user's target mech
            var weapon = AnsiConsole.Prompt(
                new SelectionPrompt<string>()
                    .Title("Select target [green]mech:[/]")
                    .PageSize(10)
                    .MoreChoicesText("[grey](Move up and down to reveal more weapons)[/]")
                    .AddChoices(weaponNames));

            // Echo the mech to be fired out in the console
            AnsiConsole.WriteLine($"Selecting {weapon}...");

            //TODO So by proxy of my above comment this is a hack!
            return weapons.FirstOrDefault(m => m.Name == weapon);
        }
    }
}
