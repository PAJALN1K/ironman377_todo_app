const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;

// Общие константы и переменные
const importanceColor = {
    high: 'red',
    medium: 'orange',
    low: 'yellow',
    unmarked: 'grey'
}

const taskList = document.querySelector('.task-list');


// Отображение задачи
const taskCreation = function(taskId, taskName, taskLifesphere, taskImportance, taskState) {
    // Создание главных родителей-дивов
    let task = document.createElement('div');
    task.classList.add('task');
    task.id = "task-" + taskId;
    taskList.appendChild(task);

    let taskPartI = document.createElement('div');
    taskPartI.classList.add('task-part');
    taskPartI.classList.add('part-1');
    task.appendChild(taskPartI);

    let taskPartII = document.createElement('div');
    taskPartII.classList.add('task-part');
    taskPartII.classList.add('part-2');
    task.appendChild(taskPartII);

    // Наполнение дива part-1
    // <input class="task-completion-square" type="image" src="img/square_grey_unfinished.png" width="20">
    let taskCompletetionButton = document.createElement('input');
    taskCompletetionButton.classList.add("task-completion-square");
    taskCompletetionButton.classList.add(taskState);
    taskCompletetionButton.type = "image";
    taskCompletetionButton.width = "20";
    let taskImgSrc = 'img/square_';
    taskImgSrc += importanceColor[taskImportance];
    taskImgSrc += '_' + taskState + '.png';
    taskCompletetionButton.src = taskImgSrc;
    taskPartI.appendChild(taskCompletetionButton);

    let taskDesc = document.createElement('input');
    taskDesc.type = "text";
    taskDesc.classList.add("task-desc");
    taskDesc.value = taskName;
    taskPartI.appendChild(taskDesc);

    // Наполнение дива part-2
    let taskLifesphereLabel = document.createElement('input');
    taskLifesphereLabel.type = "text";
    taskLifesphereLabel.classList.add("lifesphere-label");
    taskLifesphereLabel.value = taskLifesphere;
    taskPartII.appendChild(taskLifesphereLabel);

    let taskDeletionButton = document.createElement('input');
    taskDeletionButton.classList.add("task-deletion");
    taskDeletionButton.type = "image";
    taskDeletionButton.width = "27";
    taskDeletionButton.src = "img/delete_white.png";
    taskPartII.appendChild(taskDeletionButton);
};

const displayTasks = function(tasksArray) {
    for (let i = 0; i < tasksArray.length; i++) {
        taskCreation(
            tasksArray[i].id, 
            tasksArray[i].name,
            tasksArray[i].lifesphere,
            tasksArray[i].importance.toLowerCase(),
            tasksArray[i].state.toLowerCase()
        );
    };
}

const deleteAllTasks = function() {
    let tasks = document.querySelectorAll(".task");
    for (let i = 0; i < tasks.length; i++) {
        tasks[i].remove();
    };
}

const addTaskLine = document.querySelector("form");
let deleteTaskButtonElements;
let completeTaskButtonElements;
let changeTaskDescTextElements;
let markLifeSphereTextElements;
let taskSortButton;

const display = function(taskVec) {
    setTimeout(function() {
        displayTasks(taskVec);
        setTimeout(function() {
            deleteTaskButtonElements = taskList.querySelectorAll(".task-deletion");
            deleteTaskButtonElements.forEach(function(button) {
                button.addEventListener("click", taskDeletion);
            }); 

            taskElement = taskList.querySelectorAll(".task");
            taskElement.forEach(function(task) {
                task.addEventListener("contextmenu", taskMarkImportance);
            });

            completeTaskButtonElements = taskList.querySelectorAll(".task-completion-square");
            completeTaskButtonElements.forEach(function(button) {
                button.addEventListener("click", completionSquareLMB);
                button.addEventListener("contextmenu", taskMarkCompletion);
            }); 

            changeTaskDescTextElements = taskList.querySelectorAll(".task-desc");
            changeTaskDescTextElements.forEach(function(input) {
                input.addEventListener("keyup", taskRename);
            });

            markLifeSphereTextElements = taskList.querySelectorAll(".lifesphere-label");
            markLifeSphereTextElements.forEach(function(input) {
                input.addEventListener("keyup", markLifesphere);
            });

            taskSortButton = document.querySelector(".task-sort");
            taskSortButton.addEventListener("click", taskSort);
        }, 100);
    }, 100);
}

invoke('display_tasks_on_frontend').then((response) => { 
    display(response);
});

const unlisten_display = listen('display_tasks_on_frontend', (event) => {
    deleteAllTasks();
    display(event.payload);
}); 

const taskRename = function(evt) {
    if(evt.keyCode == 13) {
        evt.preventDefault();
        let renamedTask = evt.target.parentNode.parentNode;
        let renamedTaskId = Number(renamedTask.id.slice(5));
        let value = evt.target.value;
        emit("rename-task-in-backend", [renamedTaskId, value]);
        location.reload();
    }
};

const markLifesphere = function(evt) {
    if(evt.keyCode == 13) {
        evt.preventDefault();
        let markedTask = evt.target.parentNode.parentNode;
        let markedTaskId = Number(markedTask.id.slice(5));
        let lifesphere = evt.target.value;
        emit("mark-lifesphere-in-backend", [markedTaskId, lifesphere]);
        location.reload();
    }
}

const taskDeletion = function(evt) {
    let deletedTask = evt.target.parentNode.parentNode;
    let deletedTaskId = Number(deletedTask.id.slice(5));
    emit("delete-task-from-backend", deletedTaskId);
    location.reload();
};

const completionSquareLMB = function(e) {
    if (e.target.classList[1] == "unfinished") {
        let completedTask = e.target.parentNode.parentNode;
        let completedTaskId = Number(completedTask.id.slice(5));
        emit("complete-task-in-backend", completedTaskId);
        location.reload();
    } else {
        let unfinishedTask = e.target.parentNode.parentNode;
        let unfinishedTaskId = Number(unfinishedTask.id.slice(5));
        emit("unfinish-task-in-backend", unfinishedTaskId);
        location.reload();
    }
}


const taskMarkImportance = function(evt) {
    evt.preventDefault();
    document.addEventListener("click", taskImportanceHideMenu);
    document.addEventListener("contextmenu", taskImportanceMenuRBM);

    let taskImportanceMenu = document.getElementById("task-importance-menu");
    
    function taskImportanceHideMenu() {
        taskImportanceMenu.style.display = "none"
    };

    function taskImportanceMenuRBM(e) {
        if (e.target.classList[0] != "task-completion-square" && e.target == evt.target) {
            e.preventDefault();
            taskImportanceMenu.style.display = 'block';
            taskImportanceMenu.style.left = e.pageX + "px";
            taskImportanceMenu.style.top = e.pageY + "px";
        } else {
            taskImportanceHideMenu();
        };
    }

    let markTaskImportanceContextMenuButtonElements = taskImportanceMenu.querySelectorAll(".mark_importance");
    markTaskImportanceContextMenuButtonElements.forEach(function(cMButton) {
        cMButton.addEventListener("click", function(e) {
            if (evt.target.classList[0] == "task") {
                let markedTask = evt.target
                let markedTaskId = markedTask.id.slice(5);
                emit("mark-importance-of-task-in-backend", [markedTaskId, cMButton.id]);
                location.reload();
            } else {
                let markedTask = evt.target.parentNode.parentNode
                let markedTaskId = markedTask.id.slice(5);
                emit("mark-importance-of-task-in-backend", [markedTaskId, cMButton.id]);
                location.reload();
            };
        });
    });
};

const taskSort = function(evt) {
    evt.preventDefault();
    document.addEventListener("contextmenu", taskSortHideMenu);
    document.addEventListener("click", taskSortMenuLBM);

    let taskSortMenu = document.getElementById("task-sort-menu");
    
    function taskSortHideMenu() {
        taskSortMenu.style.display = "none";
    };

    function taskSortMenuLBM(e) {
        if (taskSortMenu.style.display == "none" && e.target == evt.target) {
            evt.preventDefault();
            taskSortMenu.style.display = 'block';
            
            taskSortMenu.style.right = (window.outerWidth - evt.pageX) + "px";
            taskSortMenu.style.top = evt.pageY + "px";
        } else {
            taskSortHideMenu();
        };
    }

    let taskSortContextMenuButtonElements = taskSortMenu.querySelectorAll(".cm-sort-button");
    taskSortContextMenuButtonElements.forEach(function(cMButton) {
        cMButton.addEventListener("click", function(e) {
            let sortColumn = cMButton.id.slice(8);
            emit("sort-in-backend", sortColumn);
            // location.reload();
        });
    });
};

const taskMarkCompletion = function(evt) {
    evt.preventDefault();
    document.addEventListener("click", taskCompletionHideMenu);
    document.addEventListener("contextmenu", taskCompletionMenuRBM);

    let taskCompletionMenu = document.getElementById("task-completion-menu");
    
    function taskCompletionHideMenu() {
        taskCompletionMenu.style.display = "none";
    };

    function taskCompletionMenuRBM(e) {
        if (evt.target == e.target) {
            e.preventDefault();
            taskCompletionMenu.style.display = "block";
            taskCompletionMenu.style.left = e.pageX + "px";
            taskCompletionMenu.style.top = e.pageY + "px";
        } else {
            taskCompletionHideMenu()
        };
    }

    completeTaskInContextMenu = document.getElementById("mark_as_completed");
    completeTaskInContextMenu.addEventListener("click", function(e) {
        let completedTask = evt.target.parentNode.parentNode;
        let completedTaskId = Number(completedTask.id.slice(5));
        emit("complete-task-in-backend", completedTaskId);
        location.reload();
    });

    rejectTaskInContextMenu = document.getElementById("mark_as_rejected");
    rejectTaskInContextMenu.addEventListener("click", function(e) {
        let rejectedTask = evt.target.parentNode.parentNode;
        let rejectedTaskId = Number(rejectedTask.id.slice(5));
        emit("reject-task-in-backend", rejectedTaskId);
        location.reload();
    });

    unfinishTaskInContextMenu = document.getElementById("mark_as_unfinished");
    unfinishTaskInContextMenu.addEventListener("click", function(e) {
        let unfinishedTask = evt.target.parentNode.parentNode;
        let unfinishedTaskId = Number(unfinishedTask.id.slice(5));
        emit("unfinish-task-in-backend", unfinishedTaskId);
        location.reload();
    });
};

addTaskLine.addEventListener("submit", (evt) => {
    const { elements } = evt.target;
    const data = Array.from(elements)
        .map((element) => {
        const { name, value } = element

        return { name, value }
    });

    let taskDesc = data[0].value;

    emit("add-task-to-backend", String(taskDesc))
});
