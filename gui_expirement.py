import sys
from PyQt6.QtCore import Qt
from PyQt6.QtWidgets import *

from character_dao import CharacterReader
from data_manager import GameDataManager
import gff4
from item import Item
from properties import *
from character import CharacterData
from gff4.datoolset_fields import (
    # Navigation constants to find the player data
    SAVEGAME_PLAYERCHAR,
    SAVEGAME_PLAYERCHAR_CHAR,
    SAVEGAME_CREATURE_STATS,
    # Lists of abilities and items
    SAVEGAME_STATLIST,
    SAVEGAME_EQUIPMENT,
    SAVEGAME_EQUIPMENT_ITEMS,
    SAVEGAME_SKILLLIST,
    SAVEGAME_TALENTLIST,
    SAVEGAME_SPELLLIST,
    # Field IDs within structures
    TEMPLATERESREF,
    SAVEGAME_STATPROPERTY_INDEX,
    SAVEGAME_STATPROPERTY_BASE,
    # Added
    SAVEGAME_OBJECT_NAME,
)


class MainWindow(QWidget):
    def __init__(self,  *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.setWindowTitle('Dragon Age Save Editor')

        save_path = sys.argv[1] if len(sys.argv) > 1 else "saves_for_testing/DAO.das"
        self.data_manager = GameDataManager()
        self.reader = CharacterReader(save_path, self.data_manager)
        self.char_data = CharacterData()

        main_layout = QGridLayout(self)
        self.setLayout(main_layout)

        # create a tab widget
        tab = QTabWidget(self)

        # stat page
        personal_page = QWidget(self)
        layout = QFormLayout()
        personal_page.setLayout(layout)

        self.reader.get_stats(self.char_data)

        print(self.char_data.stats)
        stat_values = [0] * 6
        stat_description = [
            'Strength',
            'Dexterity',
            'Willpower',
            'Magic',
            'Cunning',
            'Constitution'
        ]
        for i in range(6):
            pass
            stat_values[i] = QLineEdit(self)
            stat_values[i].setText(str(self.char_data.stats[stat_description[i]]))
            layout.addRow(f'{stat_description[i]}:', stat_values[i])



        # contact pane
        contact_page = QWidget(self)
        layout = QFormLayout()
        contact_page.setLayout(layout)
        layout.addRow('Phone Number:', QLineEdit(self))
        layout.addRow('Email Address:', QLineEdit(self))

        # add pane to the tab widget
        tab.addTab(personal_page, 'Stats')
        tab.addTab(contact_page, 'Tab2')

        main_layout.addWidget(tab, 0, 0, 2, 1)
        main_layout.addWidget(QPushButton('Save'), 2, 0,
                              alignment=Qt.AlignmentFlag.AlignLeft)
        main_layout.addWidget(QPushButton('Cancel'), 2, 0,
                              alignment=Qt.AlignmentFlag.AlignRight)

        self.show()

def main():


    try:

        # You need one (and only one) QApplication instance per application.
        # Pass in sys.argv to allow command line arguments for your app.
        # If you know you won't use command line arguments QApplication([]) works too.
        app = QApplication(sys.argv)

        # Create a Qt widget, which will be our window.
        window = MainWindow()
        window.show()  # IMPORTANT!!!!! Windows are hidden by default.

        # Start the event loop.
        app.exec()
        """Main entry point for the script."""


    except Exception as e:
        print(f"\nAn error occurred during processing: {e}")


if __name__ == "__main__":
    main()
# Your application won't reach here until you exit and the event
# loop has stopped.
